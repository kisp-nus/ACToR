
This folder clones C2SafterRust (https://github.com/vikramnitin9/c2saferrust). 
We copy `coreutils/` to `coreutils_actor/` and modify rust translation in it. 

# Migrating our test harness

## Summary of Changes

For each program, only keep `rust_WIP/`; remove other rust folders, like `rust`, `rust_WIP_nochunking`, `rust_WIP_randomized`.

### 1. **`filter_bad_tests.py`** (Modified)
- Added `--coreutils` flag to support `coreutils_actor/src/PROG/c/` directory structure
- Updated `parse_makefile()` to handle `$(wildcard *.c)` patterns and `-I` include dirs
- Fixed project name display for coreutils mode

### 2. **`coreutils_actor/src/cat/c/`** (New test harness)
- **Makefile**: Added `cat.ref` target with coverage flags (`-fprofile-arcs -ftest-coverage`)
- **testcmp.sh**: Copied and modified to support coreutils `.gcda` naming convention
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/cat/`
- **Result**: 72 tests, all passing

Removed `rust_WIP_*/` folders.

### 3. **`coreutils_actor/src/pwd/c/`** (New test harness)
- **Makefile**: Added `pwd.ref` target with coverage flags
- **testcmp.sh**: Copied and configured for `pwd`
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/pwd/` and fixed:
  - `cd <dir> && BINARY` → `cd <dir> && ../BINARY` (single-level relative paths)
  - `cd dir1/dir2/dir3 && BINARY` → `cd dir1/dir2/dir3 && ../../../BINARY` (nested paths)
  - Symlinks to absolute paths → `(ORIG=$PWD && cd symlink && $ORIG/BINARY)`
  - `pwd_ampersand_dir` fixed (special character in name)
- **Result**: 75 tests, all passing

Removed `rust_WIP_*/` folders.

### 4. **`coreutils_actor/src/head/c/`** (New test harness)
- **Makefile**: Added `head.ref` target with coverage flags
- **testcmp.sh**: Copied from `projects_input_BSD_manualtests/head/` and updated to support coreutils `.gcda` naming convention
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, tests03.jsonl, tests04.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/head/` (no path adjustments needed — no `cd` commands in tests)
- **Result**: 70 tests, all passing

No `rust_WIP_*/` variant folders to remove (only `rust_WIP/` existed).

### 5. **`coreutils_actor/src/split/c/`** (New test harness)
- **Makefile**: Added `split.ref` target with coverage flags
- **testcmp.sh**: Copied from `projects_input_BSD_manualtests/split/`
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, tests03.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/split/` and fixed:
  - `split_no_prefix` test: `ls x*` picked up gcov `.gcda` files starting with `x` (e.g. `xalignalloc.ref.gcda`); added `| grep -v '\.gc'` to exclude coverage files
- **Result**: 71 tests, all passing

No `rust_WIP_*/` variant folders to remove (only `rust_WIP/` existed).

### 6. **`coreutils_actor/src/uniq/c/`** (New test harness)
- **Makefile**: Added `uniq.ref` target with coverage flags
- **testcmp.sh**: Copied from `projects_input_BSD_manualtests/uniq/`
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, tests03.jsonl, tests04.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/uniq/` (no path adjustments needed)
- **Result**: 75 tests, all passing

No `rust_WIP_*/` variant folders to remove (only `rust_WIP/` existed).

### 7. **`coreutils_actor/src/tail/c/`** (New test harness)
- **Makefile**: Added `tail.ref` target with coverage flags
- **testcmp.sh**: Copied from `projects_input_BSD_manualtests/tail/` and updated to support coreutils `.gcda` naming convention
- **tests00.jsonl–tests08.jsonl, seed_tests.jsonl**: Copied from `projects_input_BSD_manualtests/tail/` and fixed:
  - Removed `fifo_input_test` (broken: reads from fifo with no writer, hangs indefinitely)
  - Removed `seek_error_simulation`, `nonregular_file_rbytes_zero`, `nonregular_file_rlines_zero` (fifo tests that hang due to backgrounded writer race condition)
  - Removed `follow_regular_file`, `follow_with_bytes`, `follow_with_lines`, `follow_multiple_files`, `follow_option_detailed` (follow-mode tests use inner `timeout 1s` or `sleep 1` exceeding harness 500ms limit)
  - Removed `obsolete_parsing_edge` (uses `-f` flag in obsolete syntax, blocks in follow mode)
- **Result**: 123 tests, all passing

No `rust_WIP_*/` variant folders to remove (only `rust_WIP/` existed).

### 8. **`coreutils_actor/src/truncate/c/`** (New test harness)
- **Makefile**: Added `truncate.ref` target with coverage flags
- **testcmp.sh**: Copied from split and modified for `truncate`
- **tests00.jsonl, tests01.jsonl, tests02.jsonl, tests03.jsonl, seed_tests.jsonl**: Written from scratch (no `projects_input_BSD_manualtests/truncate/` existed). Tests cover:
  - Basic: set/extend/shrink/zero size, create file, no-create flag
  - Suffixes: K, M, G size suffixes
  - Relative: +N, -N, underflow clamping to 0
  - Modifiers: `<` (at most), `>` (at least), `/` (round down), `%` (round up)
  - Reference file: `-r` with relative modifiers
  - IO blocks: `-o` flag
  - Error cases: invalid size, missing args, permission denied, nonexistent dir/ref
  - Content verification: preserved after extend, truncated after shrink
- **Result**: 60 tests, all passing

No `rust_WIP_*/` variant folders to remove (only `rust_WIP/` existed).

### Key Insight
The original BSD tests used `BINARY` placeholder assuming it runs in the same directory. For coreutils, the test harness creates a symlink `./main` in the working directory, so after `cd`, the relative path to `BINARY` must be adjusted based on directory depth and symlink targets.