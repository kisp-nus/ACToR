# Test Coverage Report for tail

**Date**: 2026-05-05

## Coverage Results

### Before
- **tail.c**: 42.34% of 973 lines executed
- **Overall**: 23.32% of all lines

### After
- **tail.c**: 57.76% of 973 lines executed  
- **Overall**: 33.95% of all lines

### Improvement
- **tail.c improvement**: +15.42 percentage points
- **Overall improvement**: +10.63 percentage points
- **Tests added**: 28 new tests in tests09.jsonl

## Tests Added

The new tests cover the following areas:

### Command-Line Options (11 tests)
- `--help` and `--version` options (basic usage information)
- `-v/--verbose` option (show file headers)
- `-q/--quiet` and `--silent` options (suppress headers)
- `-z/--zero-terminated` option (null byte line separators)
- `--bytes` and `--lines` long-form options
- `--retry` option (retry on missing files)
- `-F` option (follow by name with retry)

### Follow Mode Options (5 tests)
- `--follow=descriptor` explicit mode
- `--follow=name` explicit mode
- `--max-unchanged-stats` option
- `--pid` option (follow until process dies)
- `--disable-inotify` hidden option

### Size Multipliers (2 tests)
- `-n` with `K` multiplier (lines)
- `-c` with `K` and `M` multipliers (bytes)

### Input Handling (5 tests)
- stdin via pipe
- explicit `-` argument for stdin
- multiple files mixed with stdin
- single file (no header by default)
- `--presume-input-pipe` hidden option

### Edge Cases and Boundaries (3 tests)
- forward bytes at exact file size
- forward lines at exact line count
- character device (`/dev/null`)

### Error Handling (2 tests)
- invalid argument to `-c` option
- invalid argument to `-n` option

## Analysis

The tests successfully increased coverage from 42.34% to 57.76% for tail.c, a significant improvement of over 15 percentage points. The new tests primarily target:

1. **Help and option parsing paths** - Many command-line options were previously untested
2. **Follow mode functionality** - Various follow-related options and their combinations
3. **Error handling** - Invalid input validation
4. **Edge cases** - Boundary conditions and special inputs

### Remaining Uncovered Code

Further coverage improvement is challenging because:

1. **Follow mode complexity** - Full follow mode testing requires complex setups with file watching, inotify, and long-running processes that are difficult to test within the 500ms timeout
2. **Error injection** - Some error paths require simulating I/O failures, out-of-memory conditions, or other system-level failures
3. **Library code** - Significant portions of uncovered code are in shared GNU library files (quotearg.c, xmalloc.c, etc.) that contain functions not called by tail
4. **Platform-specific code** - Some code paths are conditional on system capabilities or file types

The current test suite now provides good coverage of the main functionality and common use cases for the tail program.

## Converting Existing Coreutils Tests (`coreutils/tests/tail/`) to seed_tests.jsonl

### Converted: 50 tests → `seed_tests.jsonl`

#### From `tail.pl` (25 tests)
- Obsolete options: `-1c`, `-9c`, `-12c`, `-1l`, `-1`, `-l`
- Error cases: `-cl`, `-2cX`, `-c99999999999999999999`
- Standard `-c` and `-n` options with various values (`+N`, `-N`, `0`)
- Zero-terminated mode (`-z -n 1`, `-z -n 2`)

#### From shell scripts (8 tests)
- `tail-c.sh`: pipe input (`printf | tail -c3`), `/dev/zero` comparison
- `start-middle.sh`: reading from non-beginning of file (`read x; tail`)
- `follow-name.sh`: `--follow=name` on missing file exits with error
- `retry.sh`: `--retry` without `--follow` gives warning; missing file errors; `--follow=descriptor` and `--follow=name` without `--retry` exit immediately

#### Additional functional tests (17 tests)
- Empty file, nonexistent file, `--help`, `--version`
- `-n +1`/`-c +1` (output everything), `-n +2` (skip first line), `-c +2`/`-c +8`
- Multiple files with headers, `-q` (quiet), `-v` (verbose)
- stdin via pipe, `-n`/`-c` exceeding file size
- Invalid arguments to `-n` and `-c`

### Not converted: 24 shell scripts

| Script | Reason |
|--------|--------|
| `inotify-hash-abuse.sh` | Background `tail -F`, 9 iterations, timing |
| `inotify-hash-abuse2.sh` | Background `tail -F`, 200 iterations, timing |
| `inotify-race.sh` | Race condition test, requires GDB |
| `inotify-race2.sh` | Race between initial read and inotify watch |
| `inotify-rotate.sh` | File rotation with 50 iterations, background process |
| `inotify-rotate-resources.sh` | Inotify resource leak test, background process |
| `inotify-only-regular.sh` | Checks inotify only used for regular files, background |
| `inotify-dir-recreate.sh` | Directory removal/recreation, polling fallback |
| `F-vs-rename.sh` | Background `tail -F`, file rename tracking |
| `F-vs-missing.sh` | Background `tail -F`, waits for file to appear |
| `F-headers.sh` | Background `tail -F`, header output on create/rename |
| `truncate.sh` | Background `tail -f/-F`, file truncation detection |
| `symlink.sh` | Background process, symlink target changes |
| `pipe-f.sh` | FIFO with background writer, `tail -f` indefinitely |
| `pipe-f2.sh` | POSIX compliance, SIGPIPE, closed output descriptors |
| `pid.sh` | `--pid` option, background process monitoring |
| `wait.sh` | Missing file retry with `--follow`, background |
| `follow-stdin.sh` | Background `tail -f`, tty detection |
| `append-only.sh` | Requires root + `chattr +a` |
| `end-of-device.sh` | Requires root + 4GB+ block device |
| `big-4gb.sh` | Requires 4GB+ sparse file |
| `assert.sh` | Race condition with dev/inode reuse |
| `assert-2.sh` | Variant of assert with `-F` |
| `descriptor-vs-rename.sh` | Background `tail -f`, file rename tracking |
| `overlay-headers.sh` | Background process, inotify suspension |
| `tail-n0f.sh` | Busy-wait behavior with timing measurement |
| `tail-sysfs.sh` | Depends on `/sys/kernel/profiling` (not always available) |
| `proc-ksyms.sh` | Depends on `/proc/ksyms` (deprecated, rarely exists) |
| `retry.sh` (most parts) | Background `tail --retry --follow`, only simple non-follow parts converted |

### Common reasons for skipping

1. **Background processes**: Most `tail -f` / `tail -F` tests launch tail in the background (`& pid=$!`) and then modify files, expecting tail to react. Our JSONL harness runs a single command with a 500ms timeout.
2. **Timing/sleeps**: Tests use `retry_delay_` helper with exponential backoff (up to 12.7s) to wait for tail to produce output. Incompatible with single-shot execution.
3. **FIFOs**: Several tests create named pipes (`mkfifo`) and use background writers. Cannot be modeled as prep/target/post.
4. **Inotify-specific**: Tests verify inotify behavior (resource leaks, race conditions, polling fallback). These are Linux kernel interface tests, not pure program logic.
5. **Special privileges**: Some require root (`chattr`, block devices) or specific kernel interfaces (`/proc/ksyms`, `/sys/kernel/profiling`).
6. **Debugging tools**: `inotify-race.sh` requires GDB to inject delays.
