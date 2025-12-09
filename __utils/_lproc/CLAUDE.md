# CLAUDE.md - AI Assistant Instructions

## REQUIRED READING BEFORE ANY TASK

1. README.md — complete documentation and usage
2. lproc.py — full implementation

## Quick Reference

- All documentation is in README.md
- Converter details are in `converters.md` (see `converters/cc__stdout_r.py`)
- All implementation details are in lproc.py
- The `.lproc/` directory is auto-created on first use
- The `lptail` symlink MUST exist (see README.md installation)

## Implementation Highlights (current)

- Start: `start_lproc()` launches via `nohup setsid bash -c '<pipeline>' &` so the orchestrator bash becomes a new session/group leader. Output is redirected to persistent `.stdout/.stderr` files; input comes from persistent `.stdin` via `lptail -f`.
- Buffering: The pipeline is wrapped in `stdbuf -oL` for line-buffered stdout.
- Discovery: `find_lproc_processes()` uses a single-pass `/proc` scan to identify LProcs by matching argv tokens where `argv[0]` is `lptail` and `-f <stdin>` points at the managed `.lproc` directory. Parents/siblings/children are derived via `/proc` (no per-file `lsof`).
- Kill: `kill_lproc()` resolves the orchestrator bash PID, gets its PGID, sends `SIGTERM` to the process group (`killpg`), waits briefly with retries, and escalates to `SIGKILL` if needed. Final verification checks all related PIDs.
- Readiness: After start, a short retry loop verifies the LProc has attached and prints the lptail PID when detected.

## When Modifying Code

Key functions in lproc.py:
- `get_lproc_dir()` — returns/creates the `.lproc` data directory next to the script
- `get_lproc_file_path(name, ext)` — constructs absolute paths for `.stdin/.stdout/.stderr`
- `start_lproc(name, command)` — builds the nohup/setsid command line and launches detached
- `find_lproc_processes(stdin_file)` — resolves process info via `/proc` index
- `get_lprocs_data()` — aggregates running/inactive LProcs via a single `/proc` scan
- `kill_lproc(name)` — process-group termination with verification

Notes:
- Do not reintroduce per-file `lsof` scans; they are slower and less reliable across environments.
- Keep `lptail` name stable; discovery keys off argv[0] basename `lptail` and the exact `-f <stdin>` path.
- Favor small, focused changes that preserve current behavior and CLI.

## Testing After Changes

### CLI Commands
```bash
# Quick test (start shows a readiness verification line)
./lproc.py -s test1 "grep TEST"
echo "TEST line" >> .lproc/test1.stdin
./lproc.py -l
./lproc.py -k test1   # group-based termination
```

### Terminal UI Programs
IMPORTANT: Do not run Textual apps (hlproc.py) inside this conversation. They change terminal modes and can flood the session with events. Provide instructions for the user to run in a separate terminal.

## HLProc Program

- `hlproc.py` — Textual-based UI with:
  - LIST panel with DataTable
  - HELP panel with documentation
  - DEBUG panel for troubleshooting
  - Detail views as tabs within the main TabbedContent (not a separate Screen)
  - Input box for sending to stdin in each detail tab
  - Detail header shows the full inner `bash -c` command (user command) on the right; it is truncated visually to a single line.
  - Opening a detail tab logs a one-shot snapshot to DEBUG: PGID, LPTAIL/BASH/INNER/CMD processes and full file paths.
  - LIST columns: Name, PGID, STDIN, STDOUT, STDERR, Status, Last, Command. Last shows time since most recent edit (s/m/h).
  - LIST refresh: rows are re-sorted by most-recent edit time across stdin/stdout/stderr (newest first). The table updates in-place; each refresh removes and re-adds rows in the new order to avoid empty flashes. Selection and scroll position are preserved.
  - Sorting control: Keyboard only — press `t` to toggle between Last(desc) and Name(asc).

Keybindings (UI):
- LIST: `o` Detail, `r` Refresh, `TAB` Tabs, `q` Quit.
- DETAIL tab: `i` Focus, `r` Refresh, `w` CloseDtl, Enter to send.

Everything else is documented in README.md.
