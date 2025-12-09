# Proxies

Proxies are executable Python scripts in the `proxies/` folder that you can invoke via bracket tokens in `lproc` commands. Unlike YAML templates (which substitute text), proxies are run as programs and can add behaviors such as stream processing, monitoring, and controlled restarts.

Usage examples:

```bash
./lproc.py -s c2 "[proxies/claudix2.py] --help"
./lproc.py -s c3 "[proxies/claudix3.py]"
./lproc.py -s c4 "[proxies/claudix4.py] ...any additional arguments..."
```

Notes:
- Proxy paths are resolved relative to the location of `lproc.py`.
- Extra arguments after the bracket are passed to the proxy script.
- Proxies typically forward stdin to the underlying process and forward stdout/stderr and exit codes appropriately.

## Proxy Scripts

- `proxies/claudix2.py`
  - Purpose: Parity with the `[claudix]` template as an executable proxy.
  - Behavior: Launches the `claude` CLI with defaults (`-p --dangerously-skip-permissions --model sonnet --output-format stream-json --input-format stream-json --verbose`). Forwards stdin/stdout/stderr unchanged and returns `claude`'s exit code. Essentially an `exec` wrapper around `claude` with default flags.
  - Details: See the script for exact argument construction and execution.

- `proxies/claudix3.py`
  - Purpose: Mirror assistant text to stderr while preserving the original stdout stream.
  - Behavior: Runs `claude` with the same defaults as claudix2. Reads stdout as JSONL, forwards it unchanged to stdout, extracts assistant message text (from `message.content[]` items of type `text`), and writes a mirrored, human-readable version to stderr. Also forwards `claude`'s stderr.
  - Details: See the script for the JSON parsing and forwarding logic.

- `proxies/claudix4.py`
  - Purpose: Controlled restarts of `claude` once all pending requests complete, with ergonomic stderr previews.
  - Behavior:
    - Tracks pending requests: increments an expected counter for each valid user message sent (JSON with `{ "type": "user", "message": { "role": "user", ... } }`).
    - Tracks completions: increments a seen counter for each `{ "type": "result" }` line observed on stdout.
    - On receiving a control line containing `{ "_CLAUDIX_": "RESTART", ... }`, snapshots the current expected count and polls every 2 seconds (logging to stderr) until seen >= expected_at_trigger. Then it restarts `claude`, logs the new PID/command, removes `_CLAUDIX_` from the control message, and sends the modified message to the new process (counting it if it’s a user message).
    - For each assistant message on stdout, prints a one-line preview (first line; `...` if multi-line) to stderr.
    - Forwards stdout/stderr unchanged otherwise and logs PID/command on initial start and on restarts.
  - Details: See the script for concurrency, counters, and restart orchestration.

- `proxies/codexx4.py`
  - Purpose: Codex variant of claudix4 with the same JSONL protocol and restart control.
  - Command: `codex -m gpt5 --dangerously-bypass-approvals-and-sandbox` (plus any extra args you pass after the bracket).
  - Behavior:
    - Forwards stdout/stderr unchanged; mirrors a one-line preview of assistant text to stderr.
    - Counts expected results for each valid user message sent; increments seen when `{ "type": "result" }` arrives on stdout.
    - On `{ "_CLAUDIX_": "RESTART", ... }`, waits until all previously expected results are seen, restarts `codex`, removes the control key, and sends the modified message to the new process (counting it if a user message).
  - Details: Identical coordination logic to claudix4; see script.

## Writing Your Own Proxy

- Place the script under `proxies/` and reference it as `[proxies/your_script.py]`.
- Proxies run with the current Python interpreter and can spawn or `exec` the target process.
- Keep stdout untouched if you rely on LProc’s `.stdout` logs; write human-friendly summaries or progress to stderr.
- Use line-buffered I/O (e.g., `sys.stdout.reconfigure(line_buffering=True)`) for responsive streaming.
- When coordinating multiple messages/turns, prefer robust counters over timing assumptions.
