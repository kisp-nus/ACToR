import json
import os
import shutil
import subprocess
import random
import time
from utils import white_list_for_copy_c, white_list_for_copy_rs, white_list_for_copy_test_cases, _copy_directory, run_cc

DEBUG = False

CHECKER_TASK_PROMPT_TPL = """
<task>
You are an expert C programmer.  
Your task is to add additional test cases for the C program to improve the coverage.

---

## Project Setup
- The source **C code** is located in the **main folder**.  
- The test script is located in the **./testcmp.sh**. You should run `./testcmp.sh --help` to understand how to use the test script.
- The record of added test cases is put in `test_cases_record.md` file.
- Please ignore other unrelated files.

---

## Workflow
1. Read the C code to understand the functionalities.  
2. Focus first on **core functionalities**, then explore **edge cases**.  
3. Run `make clean && make all && ./testcmp.sh coverage` to compile the C code and get the current coverage.
4. Read the coverage report and the record of added test cases in `test_cases_record.md` to find potential missed cases.
5. Design **3** new test cases that are different from existing test cases.
6. Run `./testcmp.sh coverage` to get the new coverage. Ensure that the new coverage is higher than the previous one.
7. Clean the working directory by removing temporary files and scripts, temporary test cases, and backup files.
8. Update the record of added test cases in `test_cases_record.md` file to summarize about what test cases you added.

---

## Test Case Format
- Unit tests are supposed to be stored in `testsXX.jsonl` files. The `testcmp.sh` script reads these files and runs test cases.
- Each line in this JSONL file is a test case in JSON format. It includes the test description and test inputs, without the expected outputs. 
    - For side-effect free programs, each line has this format `{"name": "test_name", "description": "test_description", "alias_name": "alias_name", "args": ["arg1", "arg2", ...], "idx": 1}`.
    - For side-effectful programs, each line has this format `{"name": "test_name", "description": "test_description", "alias_name": "alias_name", "check_file": false, "cmd_prep": "command_to_prepare_files", "cmd_target": "command_to_test_the_program", "cmd_post": "command_to_cleanup_files", "idx": 1}`. In `cmd_target`, `BINARY` is the placeholder for the path to the binary; you can only use `BINARY`, but not a binary name or path. Note that you may need to prepare files in `cmd_prep`; you may need to clean files in `cmd_post`.
- "alias_name" is the name of the alias to run the program under. By default, leave it as an empty string. If not specified, the binary will be run with its name. This is useful when we need to run the program under different names.
- Additionally, if the program changes file content and we need to compare the file contents, you can set `check_file` to `true` and use a list of commands in `cmd_target`. The first element is the command involving `BINARY`, the remaining are `cat` commands to cat file content so that the test script can compare. If `check_file` doesn't exist or it's not `true`, the test script will treat `cmd_target` as a single command. One example of the list `cmd_target` is `["BINARY -v 1 file1.txt file2.txt > output.txt", "cat output.txt"]`.
- The commands to run are in the same directory as the test script, as well as the binary. This applies to "args", "cmd_prep", "cmd_target", and "cmd_post". That means
    - If you want to cd to a different directory to run the binary, use `cd ./<dir_name> && <command>`, especially for each element in "cmd_target". Take care of the relative path of the binary once the current directory is changed. For example, once you cd into a subfolder, use `../BINARY <args>`, like `cd ./subfolder && ../BINARY <args>`.
    - For "cmd_prep" and "cmd_post", remember that their current directory is the program folder.
- Each JSONL file should have at most 15 test cases. If that file has more than 15 tests, please create a new `testsYY.jsonl` file where the `YY` index is incremented by 1 from the largest `XX` index.

---

## Double-check Before Ending the Task
1. There should be exactly 3 new test cases added to the JSONL file. You should run `./testcmp.sh` and the number of test cases will be shown. There should be `<current_test_cases_number> + 3` test cases in total.
2. The 3 new test cases should be different from existing test cases. You should check this by reading the content of the test cases.
3. The added tests must be valid for the C code. You should run `make clean && make all` and then run `./testcmp.sh compare ./xxx.out(compiled from C code)`. It must show `All tests passed!`. If this check fails, please check if the differences are rooted in inherent non-determinism (e.g., random number generation, time-based operations, file-related operations, etc.).

---

## Constraints
- When adding new test cases, you should carefully read the `Test Case Format` above and follow the test script.
- Before ending the task, you MUST follow the `Double-check Before Ending the Task` rules above to check step by step. If **any of the steps fail**, you MUST redo the task and fix the test cases**.
- When editing `test_cases_record.md`, you should always append your summary at the end of the file with a clear separation from previous rounds.
- When editing `test_cases_record.md`, **do not wrap code sections in triple backticks (` ``` `)**. You should insert code sections directly, without markdown fencing.
- You MUST only work in current `sandbox/` folder, don't touch any files outside.
</task>

--- v<version> ---
"""

CHECKER_TASK_PROMPT_TPL_FOR_BSD = """
<task>
You are an expert C programmer.
Please add more tests for the C program in the current folder. The goal is to increase the line coverage of the C program. Please refer to the `UNIT TEST FORMAT` section below for the test format.

GOALS OF ADDING TESTS:
1. Increase the line coverage of the C program.
2. Should not be duplicated with existing tests.
3. Are valid tests. Run `./testcmp.sh compare ./xxx`. The running result must show the C program pass all the tests. Internally, it means that comparing two different versions of the C program (./xxx.ref and ./xxx) should not result in any differences.

RESOURCES FOR ADDING TESTS:
- C source code files: `*.c` and `*.h` files, for understanding the functionalities.
- `Makefile` for building the program (`make all`) and clean old coverage files (`make clean`).
- `testsXX.jsonl`: existing tests.
- `testcmp.sh` for reporting the coverage of the program by existing tests. Usage: `./testcmp.sh coverage`.
- `xx.gcov` file after collecting coverage, to check which lines are not covered.

TASK:
Add 3 more tests to `testsXX.jsonl` where `XX` is the largest index among all the `testsXX.jsonl` files. If that file has more than 15 tests, please create a new `testsYY.jsonl` file where the `YY` index is incremented by 1 from the largest `XX` index. These 3 tests should be valid and increase the line coverage of the C program.

IMPORTANT: If there are indeed differences between the two versions of the C program, Please check if the differences are rooted in inherent non-determinism (e.g., random number generation, time-based operations, etc.). If so, you can modify test script to relax the output comparison ONLY for those tests failing due to non-determinism.
---

## Test Case Format
Unit tests are supposed to be stored in `testsXX.jsonl` files. The `testcmp.sh` file reads these files and runs test cases.

Each line in this JSONL file is a test case in JSON format. It includes the test description and test inputs, without the expected outputs. 
- For side-effect free programs, each line has this format `{"name": "test_name", "description": "test_description", "alias_name": "alias_name", "args": ["arg1", "arg2", ...], "idx": 1, "norm_rules": []}`.
- For side-effectful programs, each line has this format `{"name": "test_name", "description": "test_description", "alias_name": "alias_name", "check_file": false, "cmd_prep": "command_to_prepare_files", "cmd_target": "command_to_test_the_program", "cmd_post": "command_to_cleanup_files", "idx": 1, "norm_rules": []}`. In `cmd_target`, `BINARY` is the placeholder for the path to the binary; you can only use `BINARY`, but not a binary name or path. Note that you may need to prepare files in `cmd_prep`; you may need to clean files in `cmd_post`.

"alias_name" is the name of the alias to run the program under. By default, leave it as an empty string. If not specified, the binary will be run with its name. This is useful when we need to run the program under different names.

Additionally, if the program changes file content and we need to compare the file contents, you can set `check_file` to `true` and use a list of commands in `cmd_target`. The first element is the command involving `BINARY`, the remaining are `cat` commands to cat file content so that the test script can compare. If `check_file` doesn't exist or it's not `true`, the test script will treat `cmd_target` as a single command. One example of the list `cmd_target` is `["BINARY -v 1 file1.txt file2.txt > output.txt", "cat output.txt"]`.

Optional: shared setup for time/IO-sensitive tests
- Field: `shared_prep` (boolean, default `false`)
- When `true`, the harness runs `cmd_prep` once, then runs the reference binary and the test binary against the same prepared state, and finally runs `cmd_post` once. This avoids nondeterministic differences (e.g., timestamps) that arise when creating inputs twice.
- When `false` or omitted, each run performs its own `cmd_prep`/`cmd_post` as before.

The commands to run are in the same directory as the test script, as well as the binary. This applies to "args", "cmd_prep", "cmd_target", and "cmd_post". That means
- If you want to cd to a different directory to run the binary, use `cd ./<dir_name> && <command>`, especially for each element in "cmd_target". Take care of the relative path of the binary once the current directory is changed. For example, once you cd into a subfolder, use `../BINARY <args>`, like `cd ./subfolder && ../BINARY <args>`.
- For "cmd_prep" and "cmd_post", remember that their current directory is the program folder.

"norm_rules" is a list of dict, each dict has "pattern" and "replacement" fields. These rules are regex patterns and will be used to replace the strings by `sed` in the testing script. No need to normalize the binary name and the reference binary name since they're always the same: either "alias_name" or "main". 
Add normalization rules to normalize unnecessary difference between 2 outputs when comparing using `testcmp.sh`. Use `{progname}` as the placeholder for the program name. One example is removing the leading `./progname:`. By default, add the below 6 rules for any error handling tests.
+++json
[{"pattern": "\\./{progname}:", "replacement": ""}, {"pattern": "{progname}:", "replacement": ""},{"pattern": "\\./{progname}", "replacement": ""}, {"pattern": "{progname}", "replacement": ""},{"pattern": "\\./mktemp:", "replacement": ""}, {"pattern": "mktemp:", "replacement": ""},{"pattern": "\\./mktemp", "replacement": ""}, {"pattern": "mktemp", "replacement": ""}]
+++
Here replace `mktemp` with the real program name, like arch or fmt. Don't put it in quotes or brackets.

!!!IMPORTANT: Always remember to add at least these 6 rules for each test.

Each JSONL file should have at most 15 test cases. If that file has more than 15 tests, please create a new `testsYY.jsonl` file where the `YY` index is incremented by 1 from the largest `XX` index.

---

## Double-check Before Ending the Task
1. There should be exactly 3 new test cases added to the JSONL file. You should run `./testcmp.sh` and the number of test cases will be shown. There should be `<current_test_cases_number> + 3` test cases in total.
2. The 3 new test cases should be different from existing test cases. You should check this by reading the content of the test cases.
3. The added tests must be valid for the C code. You should run `make clean && make all` and then run `./testcmp.sh compare ./xxx.out(compiled from C code)`. It must show `All tests passed!`. If this check fails, please check if the differences are rooted in inherent non-determinism (e.g., random number generation, time-based operations, file-related operations, etc.).

---

## Constraints
- When adding new test cases, you should carefully read the `Test Case Format` above and follow the test script.
- Before ending the task, you MUST follow the `Double-check Before Ending the Task` rules above to check step by step. If **any of the steps fail**, you MUST redo the task and fix the test cases**.
- When editing `test_cases_record.md`, you should always append your summary at the end of the file with a clear separation from previous rounds.
- When editing `test_cases_record.md`, **do not wrap code sections in triple backticks (` ``` `)**. You should insert code sections directly, without markdown fencing.
- You MUST only work in current `sandbox/` folder, don't touch any files outside.
</task>


--- v<version> ---
"""


def validate_test(proj_name: str, work_dir: str, worker_num: int):
    """
    Validate test cases by:
    1. Compiling C code and running testcmp.sh to ensure C code equals reference
    2. Checking test number follows 3+k rule where k is number of test cases in last iteration
    3. If validation fails, recover by removing current test cases and loading from last iteration
    4. If validation passes, save memory to save folder for future recovery
    
    Returns:
        tuple: (is_valid: bool, message: str)
    """
    sandbox_dir = f"{work_dir}/sandbox/"
    
    try:
        ### Step 1: Validate C code compilation and testcmp.sh comparison
        if DEBUG: print(f"[INFO:{proj_name}] Validating C code compilation and testcmp.sh comparison...")
        
        # Copy the src to sandbox (if not already there)
        # copy_src_to_sandbox(benchmark_name, sandbox_dir)
        
        # Build the C code
        output = subprocess.run(f"make clean && make all", cwd=sandbox_dir, shell=True, capture_output=True)
        if output.returncode != 0:
            if DEBUG: print(f"[WARNING] C code compilation failed.")
            _recover_test_cases(work_dir)
            return False, f"C code compilation failed: {output.stderr.decode('utf-8')}"
        
        # Ensure reference binary exists
        # assert os.path.exists(f"{sandbox_dir}/{proj_name}.ref"), f"Reference binary not found"
        
                
        # remove all the file or folder with name main
        target = os.path.join(sandbox_dir, "main")
        # remove if it exists, regardless of being a file or folder
        if os.path.exists(target):
            if os.path.isdir(target):
                shutil.rmtree(target)  # removes directories recursively
            else:
                os.remove(target)      # removes files
                
        # Run testcmp.sh to compare with reference
        # First, check if the binary with name proj_name exists; if not, use 'binary1'
        binary_path = os.path.join(sandbox_dir, proj_name)
        if not os.path.exists(binary_path):
            # Fallback to 'binary1' if 'proj_name' binary doesn't exist
            fallback_name = "binary1"
            binary_path = os.path.join(sandbox_dir, fallback_name)
            if not os.path.exists(binary_path):
                if DEBUG: print(f"[WARNING] Neither {proj_name} nor {fallback_name} binary found in sandbox. Compilation may have failed.")
                _recover_test_cases(work_dir)
                return False, f"Neither {proj_name} nor {fallback_name} binary found after compilation."
            compare_bin_arg = f"./{fallback_name}"
        else:
            compare_bin_arg = f"./{proj_name}"

        output = subprocess.run(f"./testcmp.sh compare {compare_bin_arg}", cwd=sandbox_dir, shell=True, capture_output=True)
        stdout_str = output.stdout.decode("utf-8")
        
        if not ("All tests done." in stdout_str or "All tests passed!" in stdout_str):
            if DEBUG: print(f"[WARNING] Testcmp.sh comparison failed.")
            _recover_test_cases(work_dir)
            return False, f"Testcmp.sh comparison failed: {stdout_str}"
        
        ### Step 2: Check test number follows 3+k rule
        if DEBUG: print(f"[INFO:{proj_name}] Validating test count follows 3+k rule...")
        current_test_count = _count_test_cases_in_directory(sandbox_dir)
        
        # For subsequent iterations, should follow 3+k rule
        last_iter_test_cases_dir = f"{work_dir}/test_cases/"
        assert os.path.exists(last_iter_test_cases_dir), f"Last iteration test cases not found"
        last_test_count = _count_test_cases_in_directory(last_iter_test_cases_dir)
        expected_min_count = 3 + last_test_count
        if current_test_count != expected_min_count:
            if DEBUG: print(f"[WARNING:{proj_name}] Test count validation failed. Expected {expected_min_count} (3+{last_test_count}), got {current_test_count}")
            _recover_test_cases(work_dir)
            return False, f"Test count validation failed. Expected {expected_min_count}, got {current_test_count}"

        _save_test_cases(work_dir)
        return True, f"Test validation successful. Test count: {current_test_count}"
        
    except Exception as e:
        if DEBUG: print(f"[ERROR] Validation failed with exception: {e}")
        _recover_test_cases(work_dir)
        return False, f"Validation failed with exception: {str(e)}"


def _count_test_cases_in_directory(directory: str) -> int:
    """Count total number of test cases in all JSONL files in directory."""
    total_count = 0
    
    if not os.path.exists(directory):
        return 0
    
    for filename in os.listdir(directory):
        if filename.startswith('tests') and filename.endswith('.jsonl'):
            filepath = os.path.join(directory, filename)
            with open(filepath, 'r') as f:
                for line in f:
                    line = line.strip()
                    if line:  # Skip empty lines
                        try:
                            json.loads(line)  # Validate JSON
                            total_count += 1
                        except json.JSONDecodeError:
                            continue  # Skip invalid JSON lines
    
    return total_count


def _recover_test_cases(work_dir: str):
    """Recover by removing current test cases and loading from last iteration."""
    if DEBUG: print(f"[INFO] Recovering from last iteration...")
    
    # Remove current test cases from sandbox
    sandbox_dir = f"{work_dir}/sandbox/"
    for filename in os.listdir(sandbox_dir):
        if filename.startswith('tests') and filename.endswith('.jsonl'):
            os.remove(f"{sandbox_dir}/{filename}")
            if DEBUG: print(f"[INFO] Removed {filename} from sandbox")
    
    # Load test cases from test cases folder
    last_iter_test_cases_dir = f"{work_dir}/test_cases/"
    assert os.path.exists(last_iter_test_cases_dir), f"Last iteration test cases not found"
    # Copy test cases back to sandbox
    _copy_directory(last_iter_test_cases_dir, sandbox_dir, whitelist=white_list_for_copy_test_cases)
    

def _save_test_cases(work_dir: str):
    """Save current memory and test cases for future recovery."""
    # Ensure test checkpoints directory exists and is up to date
    test_cases_dir = f"{work_dir}/test_cases/"
    assert os.path.exists(test_cases_dir), f"Test cases not found"
    
    # Copy current test cases from sandbox to checkpoint
    sandbox_dir = f"{work_dir}/sandbox/"
    _copy_directory(sandbox_dir, test_cases_dir, whitelist=white_list_for_copy_test_cases)
    

def discriminator(proj_name: str, project_instance: str, work_dir: str, iter_num: int, is_bsd: bool = False):
    if DEBUG: print(f"[INFO:{project_instance}] --- TestGen is running ---")
    if is_bsd:
        task_prompt = CHECKER_TASK_PROMPT_TPL_FOR_BSD
    else:
        task_prompt = CHECKER_TASK_PROMPT_TPL
    worker_num = 0
    while worker_num < 3:
        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} is running...")
        run_cc(work_dir, f"{project_instance}_discriminator_iter_{iter_num}_worker_{worker_num}", task_prompt.replace("<version>", f"{worker_num}").replace("<current_test_cases_number>", f"{15 + (iter_num - 1) * 3}"), _sanity_check=False)
        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} finished running.")
        is_valid, output = validate_test(proj_name, work_dir, worker_num)
        if is_valid:
            if DEBUG: print(f"[INFO:{project_instance}] Validation passed.")
            break
        worker_num += 1
        
        if worker_num >= 3 and "Mismatch count validation failed" in output:
            if DEBUG: print(f"[INFO:{project_instance}] TestGen failed. Worker runs out of 3 times.")
            break        
    if DEBUG: print(f"[INFO:{project_instance}] --- TestGen finished running ---")
