# Test Coverage Report for `cat`

## Summary

| Metric | Value |
|--------|-------|
| Coverage before | 30.35% (397/1308 lines) |
| Coverage after | 38.99% (495/1283 lines) |
| Improvement | +8.6 pp |
| Tests before | 72 (across tests00–tests02) |
| Tests after | 122 (added 50 in tests03) |
| All tests passing | Yes |

## Tests Added (tests03.jsonl)

50 new test cases were added in `c/tests03.jsonl`, targeting specific uncovered code
paths. The tests are grouped by the coverage gaps they address.

### 1. Flag-parsing paths in `cat.c`

Tests for flags `-A`, `-E`, and `-T` used alone (without `-v`), which exercise
distinct code paths from the existing `-e` and `-t` tests.

| Test name | Flags | Lines covered |
|-----------|-------|---------------|
| `show_all_flag_A` | `-A` | cat.c:617–621 |
| `show_ends_flag_E` | `-E` | cat.c:623–625 |
| `show_tabs_flag_T` | `-T` | cat.c:627–629 |
| `show_tabs_T_no_v` | `-nT` | cat.c:467–470 (non-quoting tab path) |
| `long_options_number` | `--number` | getopt long-option parsing |
| `long_options_number_nonblank` | `--number-nonblank` | getopt long-option parsing |
| `long_options_squeeze_blank` | `--squeeze-blank` | getopt long-option parsing |
| `long_options_show_ends` | `--show-ends` | getopt long-option parsing |
| `long_options_show_tabs` | `--show-tabs` | getopt long-option parsing |
| `long_options_show_nonprinting` | `--show-nonprinting` | getopt long-option parsing |
| `long_options_show_all` | `--show-all` | getopt long-option parsing |

### 2. Non-quoting mode formatting combinations

Tests for `-E`, `-T`, `-s`, `-n`, `-b` in various combinations without `-v`, exercising
the non-quoting branch of `cat()`.

- `E_flag_with_blanks`, `E_flag_with_number_blanks`, `T_flag_multiple_tabs`
- `nT_combo`, `sE_combo`, `bE_combo`, `nE_combo`, `sTE_combo`
- `A_flag_with_blanks_squeeze`, `nET_combo`, `bsET_combo`
- `tab_without_nonprinting_squeeze`
- `tab_in_show_v_no_show_tabs` (tab passthrough in `-v` without `-T`)

### 3. CRLF / carriage-return handling

Tests targeting the `pending_cr` state machine in `cat()`, which handles CR characters
at buffer boundaries when `show_ends` is active.

| Test name | What it covers |
|-----------|----------------|
| `crlf_show_ends_no_v` | `pending_cr` path in non-quoting mode (cat.c:474–481) |
| `crlf_show_ends_with_v` | CR displayed as `^M` in quoting mode |
| `crlf_at_eof_no_newline` | `pending_cr` at program exit (cat.c:803–805) |
| `crlf_multiple_lines_E` | Multiple CRLF lines with `-nE` |
| `cr_only_no_lf_E` | CR without following LF |
| `crlf_with_A_flag` | CRLF with `-A` (show-all) |

### 4. Version output

| Test name | Lines covered |
|-----------|---------------|
| `version_flag` | version-etc.c:61–93 (23 lines), propername-lite.c (4 lines), c-strcasecmp.c (12 lines), localcharset.c (5 lines), c-ctype.h (6 lines), cat.c:633 |

### 5. copy_cat() and file-to-file copy path

Tests using the `check_file` harness mode to redirect output to a regular file,
which triggers the `copy_cat()` -> `copy_file_range()` zero-copy path.

| Test name | What it covers |
|-----------|----------------|
| `copy_cat_file_to_file` | copy_cat(), copy_file_range() (cat.c:504–522, copy-file-range.c) |
| `copy_cat_multiple_files` | Multiple files through copy_cat |
| `copy_cat_large_file` | Larger file through copy_cat |
| `copy_cat_empty_file` | copy_file_range returning 0 (cat.c:521–522) |
| `copy_cat_to_dev_null` | Output to non-regular file (falls back to simple_cat) |

### 6. Input-is-output detection

| Test name | What it covers |
|-----------|----------------|
| `input_is_output` | cat.c:709–724, fcntl.c (F_GETFL), O_APPEND detection |
| `input_is_output_with_flags` | Same path with formatting flags active |

### 7. Output buffer overflow in cooked mode

Large files that produce output exceeding the buffer size, triggering the
multi-write flush loop in `cat()`.

| Test name | What it covers |
|-----------|----------------|
| `large_highascii_cooked` | Buffer overflow with high-ASCII + `-v` (cat.c:253–268) |
| `large_control_chars_cooked` | Buffer overflow with control chars + `-v` |
| `large_file_numbered_cooked` | Large file with `-n` |
| `large_file_show_ends` | Large file with `-E` |
| `large_file_show_tabs` | Large file with `-T` |
| `large_file_all_flags` | Large file with `-A` |

### 8. Miscellaneous

- `dev_null_input`, `dev_null_with_flags` — edge case: empty device input
- `stdin_binary_mode` — stdin with null bytes, no formatting
- `high_ascii_with_A` — high ASCII range with `-A`
- `multiple_stdin_with_E` — repeated stdin references with formatting
- `file_no_read_perm_cooked` — permission denied in cooked mode

## Why 50% Coverage Is Not Achievable

Reaching 50% requires covering 642 of 1283 executable lines. After exhaustive
test development, 495 lines (38.99%) are covered. The remaining 788 uncovered
lines fall into categories that cannot be exercised through the `cat` binary.

### Breakdown of unreachable lines

| Category | Lines | % of uncovered |
|----------|------:|---------------:|
| Library functions never called by `cat` | 634 | 80.5% |
| Version output switch cases (wrong author count) | 53 | 6.7% |
| Error handling paths (require I/O fault injection) | 88 | 11.2% |
| Dead code and extreme edge cases | 13 | 1.6% |
| **Total** | **788** | **100%** |

### 1. Library functions never called by `cat` (634 lines)

The `cat` binary is compiled from ~40 source files, many of which are shared GNU
utility libraries. Most functions in these libraries are never invoked by `cat`:

| File | Uncovered | Reason |
|------|----------:|--------|
| quotearg.c | 331 | 20+ wrapper functions and 8 quoting styles (c, escape, locale, custom, etc.) that `cat` never uses. `cat` only calls `quotearg_n_style_colon` with `shell_escape_quoting_style`. |
| xmalloc.c | 81 | `x2nrealloc`, `xcalloc`, `xrealloc`, `xpalloc`, etc. — memory utilities not called by `cat`. |
| c-ctype.h | 80 | `c_isalpha`, `c_isdigit`, `c_ispunct`, etc. — character classification functions. `cat` does its own byte-level character handling. |
| fcntl.c | 36 | fcntl emulation layer for non-POSIX systems. On Linux, most code paths are skipped. |
| stdbit.h | 22 | Bit manipulation functions (`stdc_leading_zeros`, etc.) not used by `cat`. |
| ialloc.h | 17 | Integer-safe allocation wrappers (`ireallocarray`, `icalloc`) not called. |
| mbrtoc32.c | 17 | Multibyte-to-char32 conversion. `cat` processes raw bytes. |
| setlocale_null*.c | 21 | Locale query functions not called by `cat`. |
| fseeko.c | 11 | `fseeko` wrapper never invoked. |
| 8 other files | 18 | Various unused utilities (hard-locale, fpurge, xalloc, wchar, etc.). |

### 2. Version output author-count switch (53 lines)

`version-etc.c` contains a switch statement with cases for 0 through 10+ authors.
GNU `cat` has exactly 2 authors (Torbjorn Granlund and Richard M. Stallman), so
only `case 2` executes. The other 10 cases and `emit_bug_reporting_address` (which
is shadowed by an inline version in `system.h`) are dead code for this binary.

### 3. Error handling paths (88 lines)

These paths require I/O failures that the test harness cannot inject:

- **Write errors** (`write_error()` in cat.c and system.h): require stdout write
  failure (disk full, broken pipe caught at specific points).
- **Read errors** (cat.c:316–319): require a file that becomes unreadable mid-read.
- **ioctl errors** (cat.c:293–300): require `FIONREAD` ioctl to fail with an
  unexpected errno.
- **Close errors** (closeout.c, close-stream.c): require `fclose(stdout)` failure.
- **Memory allocation failure** (`xalloc_die`): requires `malloc` to return NULL.
- **Various library error paths** (fclose.c, fflush.c, safe-read.c, fadvise.c,
  copy-file-range.c, xbinary-io.h, progname.c): edge cases requiring specific
  system call failures.

### 4. Dead code and extreme edge cases (13 lines)

- **`pending_cr` in quoting mode** (cat.c:375–377, 394–395, 5 lines): The global
  `pending_cr` is only set in the non-quoting code path (line 477), but lines
  375–377 and 394–395 check it in the quoting code path. Since all files in a
  single invocation use the same mode, these lines are unreachable.
- **Line counter overflow** (cat.c:146, 148, 2 lines): Requires processing more
  than 10^17 lines to overflow the 18-digit line counter.
- **`copy_file_range` error return** (cat.c:524–530, 6 lines): Requires
  `copy_file_range` to return -1 with an errno other than the handled set
  (ENOSYS, EINVAL, EBADF, EXDEV, ETXTBSY, EPERM, ENOTSUP).

### Conclusion

The maximum achievable coverage for `cat` through black-box testing is
approximately **39%**, which we have reached. The 50% threshold would require
either:

1. Removing unused library code from the compilation unit (reducing total
   executable lines from 1283 to ~650), or
2. Fault injection (e.g., `LD_PRELOAD` to intercept `write`/`read`/`malloc` to
   simulate failures), which is outside the scope of the JSONL test harness.

The primary target file `cat.c` itself has **84.8% coverage** (245/289 lines),
with only error-handling and dead-code paths remaining.
