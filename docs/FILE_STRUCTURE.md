# File Structure Management

This document explains how ACToR manages files during the translation process.

## Overview

During the working procedure of ACToR, there may be multiple backups, working sandboxes, etc. ACToR will automatically maintain the file structure at each iteration and back them up in separate folders.

## File Categories

The files are separated into 3 categories:

| Category | Description | Owner |
|----------|-------------|-------|
| **C files** | C source files, Makefile, readme, or anything that belongs to the source code | System (read-only) |
| **Rust files** | Rust translated files, Cargo.toml, or anything produced by the translator agent | Translator |
| **Specification files** | Test driver, test cases, fuzzing script, etc. | Discriminator |

### XOR Principle

All files should be modified following the **XOR** principle: **A module can read all files, but should only modify the files that belong to itself.**

For LLM coding agents, this is hard to ensure in practice. We recommend a workaround used in the validation process: *each time after module A finishes its task, first refresh all other files not belonging to A (which should not be modified) before the validation process. This prevents A from cheating by modifying other files without authorization.*

### Log Files

Besides the 3 categories, ACToR also saves `Log files` for storing the context of translator and discriminator agents. The logs are only used for debugging and recording, and should **NOT** be read by the agents themselves.

---

## Input Structure (Default Implementation)

Each project in `projects_input/` should follow this structure:

```
projects_input/
├── expr/                           # project name
│   ├── expr.c                      # source C code
│   ├── Makefile                    # makefile
│   ├── tests00.jsonl               # initial seed tests 
│   ├── norm_rules.jsonl            # normalization rules
│   ├── testcmp.sh                  # test driver script
│   └── fuzzer_template.py          # fuzzing template
├── printf/                         # another project example
│   ├── printf.c
│   ├── Makefile
│   ├── tests00.jsonl
│   ├── norm_rules.jsonl
│   ├── testcmp.sh
│   └── fuzzer_template.py
└── ...                             # More projects
```

### Required Files

| File | Description |
|------|-------------|
| `<name>.c` | The source C code to be translated to Rust |
| `Makefile` | Build script for compiling the reference binary (`<name>.ref`) |
| `tests00.jsonl` | Test cases in JSONL format (auto-discovered `testsXX.jsonl` pattern) |
| `testcmp.sh` | Test comparison script that compares translated binary with reference |
| `norm_rules.jsonl` | Normalization rules for output comparison (e.g., strip binary names) |

### Optional Files

| File | Description |
|------|-------------|
| `fuzzer_template.py` | Template for differential fuzzing between C and Rust binaries |

### Test Case Format (`testsXX.jsonl`)

Each line in the JSONL file represents one test case:

```json
{"name": "add_pos_neg", "description": "Adding positive and negative", "alias_name": "", "args": ["3", "+", "-2"], "idx": 1}
{"name": "div_simple", "description": "Dividing two numbers", "alias_name": "", "args": ["6", "/", "2"], "idx": 2}
```

Fields:
- `name`: Unique test case identifier
- `description`: Human-readable description
- `alias_name`: Binary alias name for testing (empty string uses "main")
- `args`: Array of command-line arguments
- `idx`: Test case index

### Normalization Rules Format (`norm_rules.jsonl`)

Rules for normalizing output before comparison. You don't need to modify this in most cases.

```json
{"description": "Remove the leading ./main: in output", "pattern": "\\./main:", "replacement": ""}
{"description": "Remove the leading main: in output", "pattern": "main:", "replacement": ""}
```

---

## Working Structure (Default Implementation)

During translation, ACToR creates a working directory for each task instance. The directory name follows the pattern `<project_name>_<instance_id>` (e.g., `arch_1aaafd`).

```
.working/                             # Working root
├── task_1/                           # Task instance: <proj>_<id>
│   ├── c_files/...                   # C source files
│   ├── rs_files/...                  # Rust files
│   ├── test_cases/...                # Specification files
│   ├── log_files/...                 # Agent logs (for debugging only)
│   └── sandbox/...                   # The actual dir agents are working
├── task_2/...                        # Another task instance
└── ...                               # More task instances
```

**Note:** All agents only have access to the `sandbox` folder.

### Directory Purposes

| Directory | Purpose |
|-----------|---------|
| `c_files/` | Original C source files. **Read-only** for all agents. |
| `rs_files/` | Rust translation output. Only translator should modify. |
| `test_cases/` | Test specifications. Only discriminator should modify. |
| `log_files/` | Agent conversation logs. Should NOT be read by agents. |
| `sandbox/` | The actual working dir. All files (except log files) will be put in this folder. |

### Generated Files (Not Required as Input)

These files are generated during the build/test process:
- `<name>.ref` - Reference binary compiled with coverage flags
- `<name>.out` - Test output binary
- `*.gcda`, `*.gcno`, `*.gcov` - Coverage data files
- `*.profraw`, `*.profdata` - LLVM profiling data

### Log File Naming Convention

Log files follow the pattern: `<project>_<instance>_<role>_iter_<N>_worker_<W>_output.log`

- `role`: `translator` or `discriminator`
- `N`: Iteration number
- `W`: Worker number (for retry attempts)

---

## Backup Structure (Default Implementation)

ACToR automatically creates backups after each iteration to enable rollback and forking.

```
.backups/                              # Backup directory root
├── task_1/                           # Backups for task instance
│   ├── iteration_0/                       # State after iteration 0
│   │   ├── rs_files/...                   # Rust files snapshot
│   │   ├── test_cases/...                 # Test cases snapshot
│   │   └── log_files/...                  # Logs up to this iteration
│   ├── iteration_1/...                    # State after iteration 1
│   ├── iteration_2/...                    # State after iteration 2
│   └── ...                                # More iterations
└── task_2/...                      # Backups for another instance
```

---

## (WIP) File Whitelist (Default Implementation)

The whitelist is used to filter files when copying/moving files between directories. In the default setting, we only whitelist the following files for each category. All other files will be ignored and cleaned after each iteration.

You can extend the whitelist if your projects (specification format) have more files to manage.

```python
white_list_for_copy_c = [
    "*.c",
    "*.h",
    "*.1",  # man page files
    "*.6",  # man page files
    "*.7",  # man page files
    "*.8",  # man page files
    "Makefile",
]

white_list_for_copy_rs = [
    "*.rs",
    "Cargo.toml",
    "Cargo.lock",
]

white_list_for_copy_test_cases = [
    "testcmp.sh",
    "norm_rules.jsonl",
    "seed_tests.jsonl",
    "tests*.jsonl",
    "fuzzer_template.py",
    "test_cases_record.md",
]

white_list_for_copy_log_files = [
    "*.log",
]
```

---

## (WIP) Customizing File Structure

If you want to design your own specification format and input structures, use the default implementation as a reference. Key considerations:

1. **Input files**: Define what files are required for your translation task
2. **Working directories**: Decide how to organize files during translation
3. **Backup strategy**: Determine what needs to be backed up between iterations
4. **Whitelist**: Update the whitelist to include your custom file types

