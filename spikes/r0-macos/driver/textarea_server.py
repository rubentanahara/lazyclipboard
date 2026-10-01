import json
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer

MAX_BODY_BYTES = 4096
INITIAL_VALUE = "AAABBB"
CARET_OFFSET = 3

PAGE = f"""<!doctype html>
<meta charset="utf-8">
<title>r0 textarea</title>
<textarea id="target" rows="6" cols="48"></textarea>
<script>
const target = document.getElementById("target");
function reset() {{
  target.value = "{INITIAL_VALUE}";
  target.focus();
  target.setSelectionRange({CARET_OFFSET}, {CARET_OFFSET});
}}
async function report() {{
  const state = {{
    value: target.value,
    caret: target.selectionStart,
    focused: document.hasFocus() && document.activeElement === target,
  }};
  const reply = await fetch("/report", {{ method: "POST", body: JSON.stringify(state) }});
  if ((await reply.json()).reset) reset();
}}
reset();
setInterval(report, 40);
</script>
"""


class State:
    latest = {"value": "", "caret": -1, "focused": False}
    reset_requested = False


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if not self.host_is_local():
            return
        if self.path == "/state":
            self.reply("application/json", json.dumps(State.latest))
        else:
            self.reply("text/html", PAGE)

    def do_POST(self):
        if not self.host_is_local():
            return
        length = int(self.headers.get("Content-Length", 0))
        if not 0 <= length <= MAX_BODY_BYTES:
            self.send_error(413)
            return
        body = self.rfile.read(length)
        if self.path == "/report":
            try:
                State.latest = json.loads(body)
            except json.JSONDecodeError:
                self.send_error(400)
                return
            reset, State.reset_requested = State.reset_requested, False
            self.reply("application/json", json.dumps({"reset": reset}))
        elif self.path == "/reset":
            State.reset_requested = True
            self.reply("application/json", "{}")

    def host_is_local(self):
        if self.headers.get("Host", "").startswith(f"127.0.0.1:{self.server.server_port}"):
            return True
        self.send_error(403)
        return False

    def reply(self, content_type, body):
        payload = body.encode()
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)

    def log_message(self, *_):
        pass


HTTPServer(("127.0.0.1", int(sys.argv[1])), Handler).serve_forever()
