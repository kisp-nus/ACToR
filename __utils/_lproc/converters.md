# Converters

This folder contains streaming converters for agent stdout/stderr files produced by LProc-managed processes. Converters always process input line-by-line so they can be used with `tail -f` for live updates.

## `converters/cc__stdout_r.py`

- Name meaning: "claude code, stdout file, file reader".
- Purpose: Pretty-print Claude-style stream-JSON (one JSON object per line) into human-readable output.
- Streaming: Reads stdin continuously and flushes after each processed line; safe to use with `tail` and `tail -f`.

### Input Format
- One JSON object per line (JSONL) with keys like `type: system|assistant|user|result`.
- `assistant.message.content[]` may contain `text` and `tool_use` items.
- `user.message.content[]` may contain `tool_result` items with `tool_use_id` mapping to a prior `tool_use`.

### Output Formats
1) Default (no color)
- Unique headers: a blank line followed by `### =====[Role] <title>=====`
- Content rendered in 4-backtick fenced code blocks at column 0 (tolerates ``` inside content).

2) Colored terminal (`--color`)
- Same headers as default, but colorized (reverse style):
  bold white text on role-colored backgrounds
  - System: cyan, Assistant: green, Tool: yellow, Tool result: magenta, User: blue, Result: bright blue, Error: red.
- Content rendered in 4-backtick fenced blocks at column 0.

### Behavior
- Processes input strictly line-by-line; partial lines are ignored until newline.
- Unknown event types are shown as structured JSON inside a code fence.
- Tool results are matched to tools by `tool_use_id` and labeled as "result from <ToolName>".

### Usage
```bash
# Pretty-print to markdown
tail /path/to/.lproc/<name>.stdout | ./converters/cc__stdout_r.py > out.md

# Pretty-print with colors to terminal
tail /path/to/.lproc/<name>.stdout | ./converters/cc__stdout_r.py --color

# Live follow with colors
tail -f /path/to/.lproc/<name>.stdout | ./converters/cc__stdout_r.py --color
```

### Notes
- Designed to be idempotent and safe for appending logs; it does not buffer or re-order lines.
- Works well alongside HLProc; you can still inspect `.stdout` directly or convert it here for readability.
