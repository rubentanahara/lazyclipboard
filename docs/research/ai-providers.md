# AI providers: what the backend needs

Research date: 2026-09-30. Each fact carries one tag:

- **docs**: read on the vendor's own documentation page.
- **live**: observed by calling the real endpoint with a deliberately fake key (this gives real error bodies without spending anything).
- **unverified**: not confirmed yet. It needs a real key, so it is checked in the provider implementation issue.

All calls are made from the Rust `ai` crate with the user's own key. There is no proxy. The key is read from the OS keychain per request and never logged.

## One trait, three implementations

The `AiProvider` trait needs three operations:

1. `complete(prompt, text) -> Result<String, AiError>`: reformat or summarise, plain text in and out.
2. `test_connection() -> Result<(), AiError>`: list models with the key, no tokens spent.
3. `default_model() -> &str`: fixed default, overridable in Settings · AI · Model.

`AiError` variants map to the designed states: `NoKey` (No Key screen), `InvalidKey` (Test Failed), `RateLimited { retry_after }`, `QuotaExhausted` (billing or spend cap, never retried), `Overloaded`, `Blocked` (content refused), `Network`, `Other`.

Retry policy: retry only `RateLimited` (when the vendor says the limit is temporary), `Overloaded`, 5xx and timeouts, with exponential backoff and jitter. Honour `retry-after` when present and treat it as the minimum wait. Never retry `InvalidKey` or `QuotaExhausted`.

Classify errors by the error body, not only the HTTP status. Two vendors return several different 429 causes, and Gemini returns 400 for a bad key.

## Anthropic

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://api.anthropic.com/v1/messages` | docs |
| Headers | `x-api-key`, `anthropic-version: 2023-06-01`, `content-type: application/json` | docs |
| Request | `{"model", "max_tokens", "messages":[{"role":"user","content":"..."}]}`; `max_tokens` is required; system prompt is the top-level `system` field | docs |
| Response text | `content` is an array of blocks; take the blocks with `type == "text"` and join their `text` | docs (SDK example) |
| Bad key | HTTP 401, `{"type":"error","error":{"type":"authentication_error","message":"API key is invalid."},"request_id":null}` | live |
| Error body | top-level `error.type` and `error.message`, plus `request_id` | docs, live |
| Error types | 400 `invalid_request_error`, 401 `authentication_error`, 402 `billing_error`, 403 `permission_error`, 404 `not_found_error`, 409 `conflict_error`, 413 `request_too_large`, 429 `rate_limit_error`, 500 `api_error`, 504 `timeout_error`, 529 `overloaded_error` | docs |
| Retry | 429 carries `retry-after`, except a spend-cap 429, which has none and carries `error.details.error_code == "enforced_spend_limit_reached"`; a spend limit the user set returns 400 `invalid_request_error` instead. Retry 429 with `retry-after`, 500, 504 and 529 | docs |
| Test connection | `GET /v1/models` with the same headers; a bad key returns the 401 above | docs, live |
| Model IDs in docs | `claude-haiku-4-5`, `claude-sonnet-5`, `claude-opus-5` | docs |
| Default | `claude-haiku-4-5` (fast, cheap) | decision, confirmed 2026-09-30 |
| Not confirmed | that the default model ID works for a real key | unverified |

## OpenAI

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://api.openai.com/v1/responses` | docs, live |
| Header | `Authorization: Bearer <key>`, `Content-Type: application/json` | docs, live |
| Request | `{"model": "...", "input": "..."}`; `input` may be a string | docs |
| Response text | walk `output[]`, take items with `type == "message"`, join the `text` of their `content[]` entries with `type == "output_text"`. Do not read `output[0]`: the array can also hold reasoning and tool items, and `output_text` exists only in the SDKs, not in the raw JSON | docs |
| Bad key | HTTP 401, `{"error":{"message":"Incorrect API key provided: ...","type":"invalid_request_error","code":"invalid_api_key","param":null}}`; `/v1/responses` adds a top-level `"status":401` | live |
| Error body | `error.message`, `error.type`, `error.code`, `error.param` | docs, live |
| Rate and billing 429s | `rate_limit_error` with code `slow_down` is temporary; `credit_balance_exhausted`, `organization_spend_limit_exceeded`, `project_spend_limit_exceeded` and `organization_usage_limit_exceeded` are billing and must not be retried. 503 has type `service_unavailable_error` and code `server_is_overloaded` | docs |
| Retry | `retry-after` may be present on a temporary 429 or a 503, never on billing errors; when absent use backoff with jitter. `x-ratelimit-*` headers report limits | docs |
| Test connection | `GET https://api.openai.com/v1/models` with the Bearer header; a bad key returns the 401 above | docs, live |
| Model IDs in docs | `gpt-5.4`, `gpt-5-nano` (described as the fastest and cheapest, suited to summarisation) | docs |
| Default | `gpt-5-nano` | decision, confirmed 2026-09-30 |
| Not confirmed | that `gpt-5-nano` accepts our request with a real key | unverified |

## Gemini

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent` | docs, live |
| Key | header `x-goog-api-key: <key>`. The query parameter `?key=` also works. Use the header so the key never lands in a URL or a log | docs (header), live (both) |
| Request | `{"contents":[{"parts":[{"text":"..."}]}]}`; optional `systemInstruction`, `generationConfig` | docs |
| Response text | `candidates[].content.parts[].text` | docs |
| Bad key | **HTTP 400**, not 401: `{"error":{"code":400,"message":"API key not valid. Please pass a valid API key.","status":"INVALID_ARGUMENT","details":[{"@type":"type.googleapis.com/google.rpc.ErrorInfo","reason":"API_KEY_INVALID",...}]}}`. Detect an invalid key by `details[].reason == "API_KEY_INVALID"`, not by the status code | live |
| Error body | `error.code` (integer), `error.message`, `error.status`, `error.details[]` (google.rpc) | live |
| Blocked prompt | `promptFeedback.blockReason` set and no candidates: map to `AiError::Blocked` | docs |
| Retry | 429, 408 and 5xx are transient; 402 (prepaid credit) is not retried. 429 status strings `RESOURCE_EXHAUSTED` on the classic API were not observed | docs |
| Test connection | `GET /v1beta/models` with the key header; a bad key returns the 400 above | docs, live |
| Which API | The newer Interactions API (`/v1beta/interactions`) is documented as an alternative to `generateContent`. It stores interactions by default (55 days paid, 1 day free) and lacks custom safety settings. We use `generateContent`, which is stateless, so no captured text is stored by Google beyond the normal request log | docs, decision |
| Model IDs in docs | `gemini-3.5-flash-lite` (fastest, cheapest), `gemini-3.8-flash`; `gemini-flash-latest` is a hot-swapped alias, avoid it | docs |
| Default | `gemini-3.5-flash-lite` | decision, confirmed 2026-09-30 |
| Not confirmed | the 429 error body on `generateContent`; that the default model works for a real key | unverified |

## Verification log

What the checks changed against the first draft of this document:

| Assumption | Finding | Source |
| --- | --- | --- |
| Gemini rejects a bad key with 401 | It returns **400** `INVALID_ARGUMENT` with `details[].reason == "API_KEY_INVALID"` | live |
| Gemini needs `?key=` | The `x-goog-api-key` header works and is used; `?key=` also works | docs, live |
| Gemini error codes come from the Interactions API table | That table covers another API; `generateContent` uses the google.rpc body | live |
| Gemini's Interactions API is a drop-in | It stores interactions by default (55 days paid, 1 day free); we use stateless `generateContent` | docs |
| OpenAI text is at `output_text` or `output[0]` | Neither in raw JSON: walk `output[]` for `message` items and their `output_text` parts | docs |
| OpenAI 429 means slow down | Four of its 429 causes are billing and must not be retried | docs |
| Anthropic 429 always has `retry-after` | A spend-cap 429 has none | docs |
| Anthropic 401 shape | Matches the docs: `error.type` `authentication_error` | live |
| OpenAI `GET /v1/models` works for Test connection | Confirmed, Bearer auth, 401 on a bad key | docs, live |

## Decisions this feeds

- **Key handling:** one keychain entry per provider (`lazyclipboard.ai.<provider>`). The Gemini key goes in the header.
- **Fixed defaults:** the three defaults above, with a model text override under Advanced. Model IDs change often, so defaults live in one constant per provider.
- **No streaming in the MVP:** the design shows a loading state and then the result, so `complete` returns one string.
- **Privacy notice:** the "Sends N items" note names the provider that will receive the text.
- **Rust crates:** plain `reqwest` with `rustls` and `serde` is enough. Vendor SDKs are not needed for one endpoint each.
- **Contract tests:** each provider gets one recorded-fixture test per error class above, using the bodies in this document.

## Still unverified (needs a real key)

1. Each default model ID succeeds on a real request.
2. Gemini's 429 body on `generateContent` and Anthropic's `stop_reason` values.
3. The exact `retry-after` behaviour on a real 429 for OpenAI and Gemini.

These are acceptance criteria on the provider implementation issues, not blockers for the PRD.
