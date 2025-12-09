# LProc - Long-Process Manager

A Python tool for managing long-running background processes that continuously process input through persistent files (not FIFO pipes).

## Overview

LProc (Long-Process) allows you to create persistent background processes that:
- Read input from a regular `.stdin` file (NOT a FIFO/named pipe)
- Process it through any command (grep, awk, sed, etc.)
- Write output to persistent `.stdout` and `.stderr` files
- Store all files in a centralized `.lproc/` directory (relative to the script)
- Run detached from the terminal using `nohup`
- **Preserve all data** even after process restarts or crashes

Each LProc consists of 3 processes (in a dedicated process group):
1. **lptail**: A custom tail process (symlink to `/usr/bin/tail`) that reads the input file
2. **bash**: The orchestrator process managing the pipeline
3. **command**: Your actual processing command (e.g., `grep ERROR`)

## Why Regular Files, Not FIFOs?

This tool **intentionally uses regular files instead of FIFO pipes** for several critical reasons:

1. **Data Persistence**: Input/output data is preserved across:
   - Process crashes
   - System reboots  
   - Manual stops/restarts

2. **Buffering**: Regular files provide unlimited buffering:
   - Writers never block (unlike FIFOs which block when full)
   - Readers can catch up at their own pace
   - No data loss if consumer is slower than producer

3. **Multiple Writers**: Multiple processes can append to `.stdin` simultaneously without coordination

4. **Debugging**: You can inspect files at any time:
   - Check input that was sent
   - Review all output produced
   - Analyze errors after the fact

5. **Resume Capability**: After a crash, you can see exactly where processing stopped

## Requirements

- Python 3.8+
- Linux/Unix environment with `/proc` filesystem
- Python packages (installed automatically):
  - `pyyaml` - For command templates configuration
  - `textual` - For the terminal UI (hlproc)

## Installation

### Option 1: Install as Python Package (Recommended)

```bash
# Clone or extract the lproc directory
cd lproc/

# Create the lptail symlink if it doesn't exist
ln -s /usr/bin/tail lptail

# Install the package
pip install -e .

# Now you can use lproc and hlproc commands globally
lproc --help
hlproc
```

### Option 2: Direct Script Usage

```bash
# Ensure Python 3 is installed
# Create the lptail symlink in the script directory
ln -s /usr/bin/tail lptail

# Make scripts executable
chmod +x lproc.py hlproc.py

# Run directly
./lproc.py --help
./hlproc.py
```

### Packaging for Transfer

To transfer lproc to another computer:

```bash
# Create a portable archive
./pack.sh

# This creates: _lproc_main_TIMESTAMP.tar.gz
# Copy to target machine and extract:
tar -xzf _lproc_main_*.tar.gz
cd lproc/
pip install -e .
```

The `.lproc/` directory will be created automatically on first use in the same location as the script.

## Usage

### Start a Long-Process
```bash
./lproc.py -s <name> "<command>"
./lproc.py --start <name> "<command>"

# Single command examples:
./lproc.py -s errorlog "grep ERROR"
./lproc.py -s counter "wc -l"
./lproc.py -s parser "awk '/WARN|ERROR/ {print \$1, \$NF}'"

# Pipeline examples:
./lproc.py -s pipeline "grep ERROR | sort | uniq -c"
./lproc.py -s multi "cat | grep -v DEBUG | awk '{print \$2}'"
```

#### Single vs Multi-Piped Commands

**Single Command:**
- `stdbuf -oL` is applied for line buffering
- stdout and stderr go directly to their respective files
- Example: `grep ERROR` → buffered output line by line

**Pipeline (Multi-Piped) Commands:**
- `stdbuf -oL` is applied to the entire command group
- Only the **final command's stdout** goes to `.stdout` file
- **All stderr from all commands** goes to `.stderr` file
- Commands are wrapped in parentheses for proper redirection
- For complex pipelines, users should handle their own buffering per command if needed

Example behavior:
```bash
# Command: "cmd1 | cmd2 | cmd3"
# stdout: Only cmd3's output
# stderr: All errors from cmd1, cmd2, and cmd3
```

#### Special Characters in Commands

Commands with quotes and special characters are automatically escaped using `shlex.quote()`:
- Safe: `grep "test's data"`
- Safe: `awk '{print $1 " -> " $2}'`
- Safe: Complex pipelines with mixed quotes
- Shell variables like `$HOME` won't expand unexpectedly

### Using Command Templates

You can define command templates in `lproc.yaml` to avoid typing long commands repeatedly:

```bash
# Use a template by wrapping its name in brackets
./lproc.py -s myproc "[claudix] --help"
./lproc.py -s logfilter "[errors]"
./lproc.py -s pipeline "[logs] | [awk-linenum]"

# Templates are expanded automatically:
# [claudix] -> claude -p --dangerously-skip-permissions --model sonnet ...
# [errors] -> grep -E '(ERROR|FATAL|CRITICAL)'
# Multiple templates in one command are supported
```

Configure templates in `lproc.yaml` (same directory as `lproc.py`):
```yaml
templates:
  claudix: "claude -p --dangerously-skip-permissions --model sonnet --output-format stream-json --input-format stream-json --verbose"
  errors: "grep -E '(ERROR|FATAL|CRITICAL)'"
  logs: "grep -E '(ERROR|WARN|INFO|DEBUG)'"
```

### Proxies

In addition to simple template substitution, LProc supports executable proxy scripts referenced with brackets. If a bracket token begins with `proxies/`, LProc runs that Python script (relative to the `lproc.py` directory) instead of substituting a string.

Examples:
```bash
./lproc.py -s c2 "[proxies/claudix2.py] --help"
./lproc.py -s c3 "[proxies/claudix3.py] -p"
./lproc.py -s c4 "[proxies/claudix4.py] ...any additional arguments..."
./lproc.py -s x4 "[proxies/codexx4.py] ...any additional arguments..."
```

Available proxies and behavior summary (see `proxies.md` for details):
- `proxies/claudix2.py`: Runs `claude` with the same defaults as the `[claudix]` template, forwarding stdin/stdout/stderr and exit code.
- `proxies/claudix3.py`: Runs `claude` and mirrors assistant message text (extracted from JSONL stdout) to stderr while leaving stdout unchanged; also forwards `claude`'s stderr.
- `proxies/claudix4.py`: Like claudix3 plus restart control. On receiving a JSON line with `{ "_CLAUDIX_": "RESTART", ... }`, waits until all previously sent user messages have corresponding `{"type":"result"}` outputs (checked every 2 seconds), then restarts `claude` and sends the modified message (with `_CLAUDIX_` removed) to the new process. Logs start/restart PIDs and a one-line assistant preview to stderr.
 - `proxies/codexx4.py`: Codex variant using `codex -m gpt5 --dangerously-bypass-approvals-and-sandbox`, with the same JSONL protocol, stderr previews, and restart-on-control-message behavior as claudix4.

For more about proxies and how to build your own, read `proxies.md`.

### List Running LProcs
```bash
./lproc.py -l
./lproc.py --list
```

Output shows:
- Process group (LPTAIL, BASH, COMMAND) with PIDs
- File paths with current sizes
- Summary of running vs inactive LProcs
- **Sorted by stdin file change time** (oldest first)

### Kill an LProc
```bash
./lproc.py -k <name>
./lproc.py --kill <name>
```

This will:
1. Send SIGTERM to the LProc's process group (all related processes)
2. Wait briefly with retries for graceful shutdown
3. Escalate with SIGKILL on the group if needed
4. Verify all processes stopped; exit with error if any remain

### Delete (Archive) LProc Files
```bash
./lproc.py -d <name>
./lproc.py --delete <name>
```

This now archives instead of permanently deleting:
1. Checks if the LProc is still running (fails if running)
2. Moves all three files (.stdin, .stdout, .stderr) into `.lproc_archive/` under a timestamped subfolder `<name>_YYYYMMDD_HHMMSS`
3. Only works on stopped LProcs — kill first if needed

### Send Data to an LProc

After starting an LProc, you'll see the full paths to the files. Use those paths to send data:

```bash
# If lproc.py is at /opt/lproc/lproc.py, files will be in /opt/lproc/.lproc/
# Send a single line
echo "test line with ERROR" >> /opt/lproc/.lproc/errorlog.stdin

# Or use the path shown when you started the LProc
echo "test line with ERROR" >> /path/to/script/.lproc/errorlog.stdin

# Send multiple lines
cat mydata.log >> /path/to/script/.lproc/errorlog.stdin

# Send continuous output
tail -f /var/log/syslog >> /path/to/script/.lproc/errorlog.stdin
```

### Monitor Output
```bash
# Watch output in real-time (use the path shown when starting)
tail -f /path/to/script/.lproc/errorlog.stdout

# Check for errors
cat /path/to/script/.lproc/errorlog.stderr

# Monitor file sizes
ls -lh /path/to/script/.lproc/errorlog.*
```

**Note**: You can run `lproc.py` from any directory, and it will always use the same `.lproc/` and `.lproc_archive/` folders relative to where the script is installed.

## HLProc (Textual UI)

HLProc is an optional Textual-based UI to browse LProcs and their files.

### Launch

Run in a separate terminal window:

```
./hlproc.py
```

### List View
- Shows running and inactive LProcs.
- Columns: Name, PGID, STDIN size, STDOUT size, STDERR size, Status, Last, Command.
- Last: time since the most recent edit among STDIN/STDOUT/STDERR (s/m/h).
- Inactive rows show sizes but no command.
- Sorting: Default sort by Last (newest first). Use `t` to toggle between Last(desc) and Name(asc).
- Refresh behavior: The list updates in place and, on each refresh, rows are removed and re-added in sorted order to keep rendering stable. Selection and scroll position are preserved across refreshes.

### Detail Tabs
- Opens as additional tabs inside the main TabbedContent (alongside LIST/HELP/DEBUG).
- One tab per LProc; reuses the tab if already open.
- Header shows the full user command (inner `bash -c` payload) on the right.
  - It is truncated with an ellipsis to keep the header to a single line.
  - Opening a detail tab logs full process details (including the full command) into the DEBUG panel.
- Tabs: STDOUT / STDERR / STDIN (last 100 lines for performance).
- Auto-scrolls to end only when content changes; preserves your scroll otherwise.
- Fixed input row at the bottom to send to STDIN (Enter or Send).
- If the LProc becomes inactive, input is disabled; files remain viewable.

### Key Bindings
- List: TAB (switch tab), Arrow keys (navigate), `o` (open detail tab), `r` (refresh list), `q` (quit).
- Detail tab: `i` (focus input), Enter in input to send, `r` (refresh current detail), `w` (close detail tab).
- Debug tab: `s` (save debug log to file). The debug panel keeps the last 200 lines and preserves cursor/scroll on update.

Extras:
- `n` (create new LProc from LIST), `k` (kill selected from LIST), `c` (choose converter in a DETAIL tab).

Notes:
- `r` is context-aware: when a detail tab is active, it refreshes that tab; otherwise it refreshes the LIST.

### Copying Text
- Terminal UI captures mouse; use your terminal’s copy shortcut with Shift while selecting:
  - Linux/Windows Terminal: Ctrl+Shift+C (copy), Ctrl+Shift+V (paste)
  - macOS: Cmd+C / Cmd+V
- Avoid right-click copy (it clicks in the app and clears selection). Alternatively, use `s` in the DEBUG tab and copy from the saved file.

## File Structure

All LProc files are stored in a centralized `.lproc/` directory located in the same directory as the `lproc.py` script:

```
/path/to/script/
├── lproc.py               # Main script
├── lptail -> /usr/bin/tail # Symlink to tail
├── lproc.yaml             # Optional: Command templates
└── .lproc/                # Auto-created data directory
    ├── <name>.stdin       # Input file (write data here)
    ├── <name>.stdout      # Processed output
    └── <name>.stderr      # Error messages
└── .lproc_archive/        # Auto-created archive directory (on first use)
    └── <name>_YYYYMMDD_HHMMSS/
        ├── <name>.stdin
        ├── <name>.stdout
        └── <name>.stderr
```

Each LProc creates 3 files in `.lproc/`:
- `<name>.stdin` - Input file (write data here)
- `<name>.stdout` - Processed output
- `<name>.stderr` - Error messages

Files persist after killing the LProc and can be archived using the delete command. This centralized approach:
- Keeps your working directories clean
- Makes it easy to find all LProc data
- Allows running `lproc.py` from anywhere while accessing the same LProcs

## Technical Details

### Why lptail?

The tool uses a custom symlink `lptail` instead of regular `tail` to:
- Avoid confusion with user-run tail processes
- Make LProc processes easily identifiable in process listings
- Ensure consistent behavior across environments

### Line Buffering

Commands are automatically wrapped with `stdbuf -oL` to ensure line-buffered output, preventing data from being stuck in buffers.

Process groups: The orchestrator is launched under a new session/process group via `setsid`, so all related processes (lptail, subshell, pipeline commands) share the same PGID. This makes termination reliable with a single group signal.

### Process Architecture

```
[.lproc/<name>.stdin] <-- Writers append here (persistent storage)
     |
     | (tail -f reads continuously)
     v
lptail -f /full/path/.lproc/<name>.stdin
     |
     | (Unix pipe - in memory)
     v
(stdbuf -oL <command>)  <-- Parentheses create subshell
     |
     ├──> [.lproc/<name>.stdout] (persistent storage)
     └──> [.lproc/<name>.stderr] (persistent storage)
```

**Process Tree Structure:**
```
bash (orchestrator; session/group leader via setsid)
├── lptail (reads stdin file)
└── bash (subshell from parentheses)
    └── actual command(s)
```

For pipelines like `cmd1 | cmd2 | cmd3`:
```
bash (main process)
├── lptail
└── bash (subshell)
    ├── cmd1
    ├── cmd2
    └── cmd3
```

**Key Points**: 
- Commands are wrapped in parentheses to create a subshell
- This ensures proper stderr collection from all pipeline stages
- Only the connection between `lptail` and commands uses Unix pipes (in-memory)
- All data storage uses regular files for persistence
- The parentheses ensure ALL stderr is captured, not just the last command's
- The entire pipeline runs in a dedicated process group for robust lifecycle management

## Converters

To pretty-print agent `.stdout` files (especially stream-JSON logs), use the converter documented in `converters.md`.

Quick start:
```bash
# Markdown output
tail /path/to/.lproc/<name>.stdout | ./converters/cc__stdout_r.py > out.md

# Colored terminal output
tail -f /path/to/.lproc/<name>.stdout | ./converters/cc__stdout_r.py --color
```

## Pretty-Print CLI

Use the built-in pretty command to view the last N lines of a task's stream (stdin/stdout/stderr) through a converter in `converters/`.

```bash
# Syntax
./lproc.py --pretty <NAME> <STREAM> <N> <CONVERTER>
# or
./lproc.py -p <NAME> <STREAM> <N> <CONVERTER>

# Examples (uses converters/cc__stdout_r.py)
./lproc.py -p mytask stdout 200 cc
./lproc.py --pretty mytask stdout -1 cc -- --color > pretty.md  # full file
```

Details:
- STREAM is one of: `stdin`, `stdout`, `stderr`.
- N: positive integer (last N lines) or `-1` (full file).
- Converter path: `converters/<CONVERTER>__<STREAM>_r.py` (run with current Python).
- Extra args must come after a literal `--` separator and are forwarded to the converter (e.g., `--color`).
- On success: prints only the converter output to stdout (no extra lines or messages).
- On failure: prints an error message to stdout and exits with a non-zero code.

## Developer Notes

### Key Components for Extension

#### 1. Process Management
- `start_lproc()`: Creates the nohup command pipeline and starts a new session/group via `setsid`
- `find_lproc_processes()`: Locates processes via a single-pass `/proc` scan (fast and deterministic)
- `kill_lproc()`: Handles group-based termination with verification and escalation

#### 2. Process Discovery (fast path)
The tool finds LProc processes by scanning `/proc` once and building an index:
1. Enumerate PIDs and read `/proc/<pid>/cmdline` tokens
2. Match processes where `argv[0]` basename is `lptail` and `argv` contains `-f <stdin>`
3. From the lptail PID, read parent PID to find the orchestrator bash
4. Find the subshell sibling and its child processes (actual commands)

Notes:
- Uses exact argv tokens; works reliably even when `lptail` is a symlink to `tail`
- All file paths are absolute (via `get_lproc_file_path()`)
- For pipelines, the subshell typically has multiple children (one per command)

#### 3. Critical Paths
- **lptail location**: `get_lptail_path()` finds lptail relative to the script location
- **LProc directory**: `get_lproc_dir()` creates and returns the `.lproc/` directory path
- **File paths**: `get_lproc_file_path()` constructs full paths for all LProc files
- **File existence check**: Prevents overwriting existing LProc files
- **Process verification**: `is_process_alive()` uses `kill(pid, 0)` to check process state

#### 4. Extension Points

To add new features, consider:

1. **New commands**: Add to the argument parser in `main()`
2. **Process monitoring**: Extend `find_lproc_processes()` for more process info
3. **File management**: Add cleanup options for `.stdin/.stdout/.stderr` files
4. **Status reporting**: Enhance `list_lprocs()` with more metrics
5. **Command validation**: Add checks in `start_lproc()` before execution

### Common Pitfalls

1. **File paths**: Always use `get_lproc_file_path()` for LProc files
2. **Directory creation**: The `.lproc/` directory is created automatically on first use
3. **Process cleanup**: Group-based termination is used; allow a short grace period before escalation
4. **lptail dependency**: Script fails if lptail symlink is missing
5. **Race conditions**: Small delays exist between process creation/termination; start adds a brief readiness check
6. **Buffer flushing**: Without `stdbuf -oL`, output may be delayed
7. **Path assumptions**: Never assume files are in current directory
8. **Pipeline buffering**: For complex pipelines, `stdbuf` only applies to the group, not individual commands
9. **Process discovery**: Subshell layer still applies; discovery walks parent/sibling/children via `/proc`

### Testing

```bash
# Test basic functionality (start prints readiness verification once attached)
./lproc.py -s test1 "grep TEST"
echo "TEST line" >> test1.stdin
cat test1.stdout
./lproc.py -l
./lproc.py -k test1   # Uses process-group termination

# Test with complex commands
./lproc.py -s complex "awk '{print NR, \$0}' | grep -E '[0-9]+'"
seq 1 10 >> complex.stdin
cat complex.stdout

# Test error handling
./lproc.py -s test1 "grep TEST"  # Should fail - files exist
./lproc.py -k nonexistent        # Should fail - doesn't exist
```

## Limitations

- Requires Linux/Unix environment with `/proc` filesystem
- Commands must handle streaming input (no seekable files)
- File cleanup is manual (by design - preserves data)
- No built-in log rotation (files grow indefinitely)
- Single machine only (no distributed processing)
- Disk space usage grows with input/output data
- `tail -f` starts from current end of file (misses data written before LProc starts)

## License

This tool is provided as-is for system administration and development purposes.
