# Task: Add Tests to Increase Line Coverage

## Objective

For the specified coreutils program, add approximately 20 new tests to increase
the **main source file** (`<program>.c`) line coverage to **80%+** as measured by
`./testcmp.sh coverage`.

Increase the coverage as much as possible.
Don't add tests that trigger C memory bugs or undefined behaviors, e.g., double
free, segfault, etc.

## Special Considerations

If each program has ADD_TEST_REPORT.md, read it, critically analyze the recommendations to further increase coverage, and decide the next actions. For `pwd`, its tests were added and accidentally deleted. Don't try to search it; but follow the report (what has done in the past) and critically think and decide the next actions.

- Remember to use `ulimit -v` to set the virtual memory limit to a reasonable value. ESPECIALLY FOR tests that may create large buffers. 
- Skip error paths if they need complex setup.
- `head`: Add tests for elide-from-end mode and old syntax with option letters.
- `pwd`: don't try the extremely deep directory, but the Logical PWD Validation Edge Cases.

## Coverage Target

The primary metric is the line coverage of **`<program>.c` only** (not the
overall coverage of all compiled files). In the gcov output from
`./testcmp.sh coverage`, look for:

```
File '<program>.c'
Lines executed:XX.XX% of NNN
```

Target: **80%+ on `<program>.c`**. Also note overall coverage for context.

## Context

The programs live under:
```
c2saferrust/coreutils_actor/src/<PROGRAM>/c/
```

Each program directory contains:
- `<program>.c` — the main source file (THIS IS THE COVERAGE TARGET)
- `*.c` — shared GNU library files (quotearg.c, xmalloc.c, etc.)
- `include/` — header files
- `Makefile` — builds `<program>` (optimized) and `<program>.ref` (coverage-instrumented)
- `testcmp.sh` — test harness script
- `testsXX.jsonl` — existing test files (JSONL format, auto-discovered)

## Steps

1. **Measure baseline coverage**
   ```bash
   cd c2saferrust/coreutils_actor/src/<PROGRAM>/c/
   ./testcmp.sh coverage
   ```
   In the gcov output, find the line for `<program>.c`:
   ```
   File '<program>.c'
   Lines executed:XX.XX% of NNN
   ```
   This is your baseline. Note it down.

2. **Understand uncovered code**
   - Read `<program>.c` to understand the program's features and flags.
   - After running coverage, examine `<program>.c.gcov` to identify uncovered
     lines (marked with `#####:`).
   - Categorize uncovered lines, including but not limited to:
     - Feature paths not yet tested (flags, options, modes)
     - Error handling (I/O failures, permission errors, etc.)
     - Edge cases (buffer boundaries, large files, special characters, etc.)
     - Dead/unreachable code

3. **Add new tests**
   - Create a new file `tests<NN>.jsonl` where `<NN>` is the next available number
     (e.g., if tests00–tests03 exist, create `tests04.jsonl`).
   - Add ~30 tests targeting the uncovered code paths in `<program>.c`.
   - Each test is a single JSON line with this format:
     ```json
     {"name": "test_name", "description": "What this tests", "cmd_prep": "setup commands", "cmd_target": "BINARY [flags] [args]", "cmd_post": "cleanup commands", "idx": 1, "norm_rules": []}
     ```
   - Key fields:
     - `cmd_prep`: Shell commands to create input files/setup
     - `cmd_target`: The command to run. Use `BINARY` as placeholder for the
       program path. Can be a string (simple mode) or JSON array (check_file mode).
     - `cmd_post`: Shell commands for cleanup (remove temp files)
     - `norm_rules`: Array of `{"pattern": "regex", "replacement": "string"}` for
       normalizing output differences (use `{progname}` placeholder for program name).
       Needed for error messages that include the program name.
     - `check_file`: Set to `true` when `cmd_target` is a JSON array. The first
       element is the main command, subsequent elements are `cat` commands to read
       output files for comparison.
   - The file MUST end with a trailing newline.

4. **Validate tests pass**
   ```bash
   ./testcmp.sh compare ./<program>.ref
   ```
   All tests (existing + new) must pass.

5. **Measure new coverage**
   ```bash
   ./testcmp.sh coverage
   ```
   Find the `<program>.c` line again and confirm it is 80%+.

6. **Write a report**
   - Append results to `c2saferrust/coreutils_actor/src/<PROGRAM>/ADD_TEST_REPORT.md`.
   - Use **append-only** mode — do not overwrite existing content in that file.
   - The report should include:
     - Before/after `<program>.c` coverage (percentage and line counts)
     - Before/after overall coverage (for context)
     - Number of tests added
     - Summary of what the new tests cover (grouped by category)
     - If 80% is hard to reach, explain why (e.g., error paths requiring fault
       injection, unreachable code, paths that need interactive input, etc.)

## Guidelines

- Focus exclusively on covering lines in **`<program>.c`**. Ignore uncovered
  lines in library files (quotearg.c, xmalloc.c, c-ctype.h, etc.) — those
  functions are not called by the program and cannot be covered through tests.
- Use `--version` and `--help` flags to cover version/help output paths.
- Use large files to trigger buffer management paths.
- Use `check_file: true` mode with file redirection to trigger file-to-file
  copy paths (where stdout is a regular file instead of a pipe).
- Use special characters (CRLF, control chars, high ASCII, null bytes) to
  exercise character handling paths.
- Use permission-denied and nonexistent files to exercise error paths.
- Tests have a 500ms timeout — avoid tests that read from stdin without
  providing input via redirection.
- Test names must be unique across ALL jsonl files for the program.

## Test Harness Reference

- `BINARY` in `cmd_target` is replaced with the actual binary path at runtime.
- The binary runs under a symlink alias (default: `main`, or set via `alias_name`).
- Output normalization applies `norm_rules` patterns via sed before comparison.
- `{progname}` in norm_rules patterns is replaced with the alias name.
