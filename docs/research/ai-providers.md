# AI providers: what the backend needs

Research date: 2026-09-30. Sources are the vendors' own docs. Each fact is tagged **verified** (read on the vendor page during this research) or **to verify** (from general knowledge or a page that did not show it). Every "to verify" item needs a live check before the provider implementation is merged.

All calls are made from the Rust `ai` crate with the user's own key. There is no proxy. The key is read from the OS keychain per request and never logged.

## One trait, three implementations

The `AiProvider` trait needs three operations:

1. `complete(prompt, text) -> Result<String, AiError>`: reformat or summarise, plain text in and out.
2. `test_connection() -> Result<(), AiError>`: list models with the key, no tokens spent.
3. `default_model() -> &str`: fixed default, overridable in Settings · AI · Model.

`AiError` variants map to the designed states: `NoKey` (No Key screen), `Unauthorized` (Test Failed), `RateLimited { retry_after }`, `Overloaded`, `Blocked` (content refused), `Network`, `Other`. Retry on `RateLimited`, `Overloaded` and 5xx with backoff; never retry other 4xx.

## Anthropic

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://api.anthropic.com/v1/messages` | verified |
| Headers | `x-api-key`, `anthropic-version: 2023-06-01`, `content-type: application/json` | verified |
| Request | `{"model", "max_tokens", "messages":[{"role":"user","content":"..."}]}`; `max_tokens` is required; system prompt is the top-level `system` field | verified |
| Response text | `content[]` blocks; take blocks with `type == "text"` | to verify |
| Errors | `{"type":"error","error":{"type","message"},"request_id"}`; 401 `authentication_error`, 403 `permission_error`, 413 `request_too_large`, 429 `rate_limit_error`, 500 `api_error`, 529 `overloaded_error` | to verify |
| Retry | `retry-after` header on 429 | to verify |
| Test connection | `GET /v1/models` with the same headers; returns `data[]`, paged with `limit`, `after_id` | verified |
| Model IDs seen in docs | `claude-haiku-4-5`, `claude-sonnet-4-6`, `claude-opus-5` | verified |
| Proposed default | `claude-haiku-4-5` (fast, cheap) | decision |

## OpenAI

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://api.openai.com/v1/responses` | verified |
| Header | `Authorization: Bearer <key>`, `Content-Type: application/json` | verified |
| Request | `{"model": "...", "input": "..."}`; `input` may be a string | verified |
| Response text | `output[]` items; do not assume the first item is the message; SDKs expose `output_text`, in raw HTTP concatenate the `output_text` content parts of `message` items | verified (guidance), to verify (exact path) |
| Chat Completions | still available, but Responses is the documented path | to verify |
| Errors | `{"error":{"message","type","code"}}`, 401 bad key, 429 rate limit or quota | to verify |
| Retry | `retry-after` and `x-ratelimit-*` headers | to verify |
| Test connection | `GET /v1/models` | to verify |
| Model IDs seen in docs | `gpt-5.4`, `gpt-5-nano` (described as the fastest, cheapest, suited to summarisation) | verified |
| Proposed default | `gpt-5-nano` | decision, verify it is current |

## Gemini

| Item | Value | Status |
| --- | --- | --- |
| Endpoint | `POST https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent` | verified |
| Key | docs' curl samples use the query parameter `?key=`; prefer the `x-goog-api-key` header so the key never appears in a URL or a log | header to verify |
| Request | `{"contents":[{"parts":[{"text":"..."}]}]}`; optional `systemInstruction`, `generationConfig` | verified |
| Response text | `candidates[].content.parts[].text` | verified |
| Blocked prompt | `promptFeedback.blockReason` set and no candidates: map to `AiError::Blocked` | verified |
| Errors | the Interactions API page lists `authentication` 401, `permission_denied` 403, `rate_limit_exceeded` 429, `quota_exceeded` 429, `service_unavailable` 503 and the body `{"error":{"code","message"}}`; the classic `generateContent` body may instead be `{"error":{"code","message","status"}}` | to verify against `generateContent` |
| Retry | 429, 408 and 5xx are transient; 402 (prepaid credit) is not retried | verified |
| Test connection | `GET /v1beta/models`, `pageSize` up to 1000 | verified |
| Model IDs seen in docs | `gemini-3.5-flash-lite` (fastest, cheapest), `gemini-3.8-flash`; `gemini-flash-latest` is a hot-swapped alias, avoid it | verified |
| Proposed default | `gemini-3.5-flash-lite` | decision |

## Decisions this feeds

- **Key handling:** one keychain entry per provider (`lazyclipboard.ai.<provider>`). Gemini's key goes in a header, never a URL.
- **Fixed defaults:** the three defaults above, with a model text override under Advanced. Model IDs change often, so defaults live in one constant per provider.
- **No streaming in the MVP:** the design shows a loading state and then the result, so `complete` returns one string.
- **Privacy notice:** the "Sends N items" note names the provider that will receive the text.
- **Rust crates:** plain `reqwest` with `rustls` and `serde` is enough. Vendor SDKs are not needed for one endpoint each.
