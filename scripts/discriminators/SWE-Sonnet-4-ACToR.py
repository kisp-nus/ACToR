import json
import os
import shutil
import subprocess
import random
from agents.cli_agent import MyCLIAgent
from agents.utils.local_env import LocalEnvironment
from agents.utils.models import AnthropicModel, OpenAIModel
import time
from utils import white_list_for_copy_c, white_list_for_copy_rs, white_list_for_copy_test_cases, _copy_directory, run_cc

DEBUG = False

SEED_TEST_CASES = 15
TEST_CASES_PER_ITERATION = 3
CHECKER_TASK_PROMPT_TPL = """
<task>
You are an expert in **C** and **Rust**.  
Your task is to add additional test cases to discover semantic mismatches between the C code and the translated Rust code.

---

## Project Setup
- The source **C code** is located in the **main folder**.  
- The test script is located in the **./testcmp.sh**. You should run `./testcmp.sh --help` to understand how to use the test script.
- The translated **Rust code** is located in the **`ts/` folder**.  
- The compiled binary of the translated Rust code is put inside `ts/target/release/xxx`, where `xxx` is the name of the C program.
- The record of added test cases is put in `test_cases_record.md` file.
- The current fuzzer template is located in the **`fuzzer_template.py`** file, which will be helpful for efficiently discovering mismatches between C code and Rust translation. You should read the file to understand how to control the fuzzer.

---

## Workflow
1. Analyze the C code and the translated Rust code to detect **semantic mismatches**.  
2. Focus first on **core functionalities**, then explore **edge cases**.  
3. Read the current test script and the record of added test cases in `test_cases_record.md` to find potential missed cases.
4. Update the fuzzer with diverse input patterns to expose mismatches where the C code and Rust translation code have different outputs.
5. Collect the best **<TEST_CASES_PER_ITERATION>** new input cases that expose mismatches between C code and Rust translation. Add the <TEST_CASES_PER_ITERATION> new test cases to the test cases file.
6. Run the new tests to compare the output of the translated Rust code with the original C program to confirm the mismatches.
7. Clean the working directory by removing temporary files and scripts, temporary test cases, and backup files. Don't remove `fuzzer_template.py`, `test_cases_record.md`, and test cases files.
8. Update the record of added test cases in `test_cases_record.md` file to summarize about **how you find the new mismatches** and **what test cases you added**.

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
1. There should be exactly <TEST_CASES_PER_ITERATION> new test cases added to the JSONL file. You should run `./testcmp.sh` and the number of test cases will be shown. There should be `<current_test_cases_number> + <TEST_CASES_PER_ITERATION>` test cases in total.
2. The <TEST_CASES_PER_ITERATION> test cases should be different from each other. You should check this by reading the content of the test cases.
3. The added tests must be valid for the C code. You should run `make clean && make all` and then run `./testcmp.sh compare ./xxx.out(compiled from C code)`. It must show `All tests passed!`. If this check fails, please check if the differences are rooted in inherent non-determinism (e.g., random number generation, time-based operations, file-related operations, etc.).
4. The added tests should reflect the differences between the C code and the Rust code. You should run `./testcmp.sh compare ./ts/target/release/xxx(compiled from Rust code)`. The Rust code should fail on all <TEST_CASES_PER_ITERATION> new test cases.

---

## Constraints
- When running the fuzzer script and the binaries, you should use `timeout` to set the timeout to 10~30 seconds. You should also set the memory limit to 5GB.
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
You are an expert in **C** and **Rust**.  
Your task is to add additional test cases to discover semantic mismatches between the C code and the translated Rust code.

---

## Project Setup
- The source **C code** is located in the **main folder**.  
- The test script is located in the **./testcmp.sh**. You should run `./testcmp.sh --help` to understand how to use the test script.
- The translated **Rust code** is located in the **`ts/` folder**.  
- The compiled binary of the translated Rust code is put inside `ts/target/release/xxx`, where `xxx` is the name of the C program.
- The record of added test cases is put in `test_cases_record.md` file.
- The current fuzzer template is located in the **`fuzzer_template.py`** file, which will be helpful for efficiently discovering mismatches between C code and Rust translation. You should read the file to understand how to control the fuzzer.

---

## Instructions
Find <TEST_CASES_PER_ITERATION> new input cases that expose mismatches between C code and Rust translation. Add these <TEST_CASES_PER_ITERATION> new test cases to `testsXX.jsonl` where `XX` is the largest index among all `testsXX.jsonl` files. If that file already has more than 15 tests, create a new `testsYY.jsonl` with `YY = XX + 1`. These <TEST_CASES_PER_ITERATION> tests must be valid (C vs C passes) and should aim to highlight mismatches between Rust and C (Rust vs C should differ if the current translation is incomplete).

You can consider finding tests that reveal mismatches using simple ways first; if can't find after one or two tries, you can refer to the coverage report or use the fuzzer template to help. Update the fuzzer template so that it has diverse input patterns to expose mismatches where the C code and Rust translation code have different outputs.

IMPORTANT: You MUST be careful about the fuzzing design so that the fuzzer behavior is not dangerous to the system. Choose to be conservative and avoid dangerous behaviors in the fuzzing script or the preprocessing and postprocessing steps.

IMPORTANT: If differences you observe are due to inherent nondeterminism (randomness, time, environment), either adjust the normalization logic for the specific test case or come up with a different test case that does not suffer from nondeterminism. Our validity requirement—`./testcmp.sh compare ./xxx` for C vs C must pass—ensures differences are not from nondeterminism. 

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
1. There should be exactly <TEST_CASES_PER_ITERATION> new test cases added to the JSONL file. You should run `./testcmp.sh` and the number of test cases will be shown. There should be `<current_test_cases_number> + <TEST_CASES_PER_ITERATION>` test cases in total.
2. The <TEST_CASES_PER_ITERATION> test cases should be different from each other. You should check this by reading the content of the test cases.
3. The added tests must be valid for the C code. You should run `make clean && make all` and then run `./testcmp.sh compare ./xxx.out(compiled from C code)`. It must show `All tests passed!`. If this check fails, please check if the differences are rooted in inherent non-determinism (e.g., random number generation, time-based operations, file-related operations, etc.).
4. The added tests should reflect the differences between the C code and the Rust code. You should run `./testcmp.sh compare ./ts/target/release/xxx(compiled from Rust code)`. The Rust code should fail on all <TEST_CASES_PER_ITERATION> new test cases.

---

## Constraints
- When running the fuzzer script and the binaries, you should use `timeout` to set the timeout to 10~30 seconds. You should also set the memory limit to 5GB.
- When adding new test cases, you should carefully read the `Test Case Format` above and follow the test script.
- Before ending the task, you MUST follow the `Double-check Before Ending the Task` rules above to check step by step. If **any of the steps fail**, you MUST redo the task and fix the test cases**.
- When editing `test_cases_record.md`, you should always append your summary at the end of the file with a clear separation from previous rounds.
- When editing `test_cases_record.md`, **do not wrap code sections in triple backticks (` ``` `)**. You should insert code sections directly, without markdown fencing.
- You MUST only work in current `sandbox/` folder, don't touch any files outside.
</task>

--- v<version> ---
"""

def prepare_agent(agent_name, model_name, working_dir, agent_id):
    if agent_name == "cli":
        Agent_Class = MyCLIAgent
    else:
        raise ValueError(f"Invalid agent name: {agent_name}")

    if model_name == "claude-sonnet-4-5-20250929":
        Model_Class = AnthropicModel
    elif model_name == "claude-sonnet-4-20250514":
        Model_Class = AnthropicModel
    elif model_name == "gpt-5-2025-08-07":
        Model_Class = OpenAIModel
    elif model_name == "gpt-5-mini-2025-08-07":
        Model_Class = OpenAIModel
    elif model_name == "gpt-4o-2024-11-20":
        Model_Class = OpenAIModel
    elif model_name == "gpt-4.1-2025-04-14":
        Model_Class = OpenAIModel
    else:
        raise ValueError(f"Invalid model name: {model_name}")

    agent_name = f"{agent_name}_{model_name}"
    agent = Agent_Class(
        agent_id=agent_id,
        model=Model_Class(model_name=model_name),
        env=LocalEnvironment(),
        sandbox_dir=f"{working_dir}/sandbox/",
        log_path=f"{working_dir}/log_files/{agent_name}_{agent_id}_output.log",
        checkpoint_path=f"{working_dir}/log_files/{agent_name}_{agent_id}_checkpoint.json",
    )
    return agent

def run_agent(agent: MyCLIAgent, task_prompt: str):
    agent.run(task_prompt)

def validate_test(proj_name: str, work_dir: str, worker_num: int, is_bsd: bool = False):
    """
    Validate test cases by:
    1. Compiling C code and running testcmp.sh to ensure C code equals reference
    2. Checking test number follows TEST_CASES_PER_ITERATION+k rule where k is number of test cases in last iteration
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
        
        ### Step 2: Check test number follows TEST_CASES_PER_ITERATION+k rule
        if DEBUG: print(f"[INFO] Validating test count follows TEST_CASES_PER_ITERATION+k rule...")
        current_test_count = _count_test_cases_in_directory(sandbox_dir)
        
        # For subsequent iterations, should follow TEST_CASES_PER_ITERATION+k rule
        last_iter_test_cases_dir = f"{work_dir}/test_cases/"
        assert os.path.exists(last_iter_test_cases_dir), f"Last iteration test cases not found"
        last_test_count = _count_test_cases_in_directory(last_iter_test_cases_dir)
        expected_min_count = TEST_CASES_PER_ITERATION + last_test_count
        if current_test_count != expected_min_count:
            if DEBUG: print(f"[WARNING:{proj_name}] Test count validation failed. Expected {expected_min_count} ({TEST_CASES_PER_ITERATION}+{last_test_count}), got {current_test_count}")
            _recover_test_cases(work_dir)
            return False, f"Test count validation failed. Expected {expected_min_count}, got {current_test_count}"

        if not is_bsd: # skip step 3 if is BSD
            ### Step 3: Check if the Rust code failed on the new test cases
            if DEBUG: print(f"[INFO] Validating Rust code failed on the new test cases...")
            output = subprocess.run(f"./testcmp.sh compare ./ts/target/release/{proj_name}", cwd=sandbox_dir, shell=True, capture_output=True)
            output_text = output.stdout.decode("utf-8")
            passed_tests = 0
            total_tests = None
            for line in output_text.split('\n'):
                if "Results:" in line and "passed" in line and "failed" in line and "out of" in line:
                    # Parse "Results: X passed, Y failed out of Z tests"
                    parts = line.split()
                    for i, part in enumerate(parts):
                        if part == "passed," and i > 0:
                            try:
                                passed_tests = int(parts[i - 1])
                            except ValueError:
                                continue
                        if part == "of" and i + 1 < len(parts):
                            try:
                                total_tests = int(parts[i + 1])
                                break
                            except ValueError:
                                continue
                    break
            
            assert total_tests == current_test_count, f"Total tests mismatch: {total_tests} vs {current_test_count}"
            if passed_tests > total_tests - TEST_CASES_PER_ITERATION and worker_num < 2:
                if DEBUG: print(f"[WARNING] Mismatch count validation failed. Expected the Rust code to fail on the new test cases, but it passed on {passed_tests} / {total_tests} tests")
                _recover_test_cases(work_dir)
                return False, f"Test count validation failed. Expected the Rust code to fail on the new test cases, but it passed on {passed_tests} / {total_tests} tests"

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
    
    # # Remove current memory (ts checkpoints)
    # current_testgen_memory_dir = f"{sandbox_dir}/test_cases_record.md"
    # assert os.path.exists(current_testgen_memory_dir), f"Current testgen memory not found"
    # os.remove(current_testgen_memory_dir)
    # print(f"[INFO] Removed current testgen memory: {current_testgen_memory_dir}")

    # # Load memory from last iteration
    # assert iter_num > 0, f"Last iteration not found"
    # last_testgen_memory_dir = f"{output_dir}/test_checkpoints/test_iter_{iter_num - 1}/test_cases_record.md"
    # assert os.path.exists(last_testgen_memory_dir), f"Last iteration memory not found"
    # # Copy memory back to sandbox
    # shutil.copy(f"{last_testgen_memory_dir}", f"{sandbox_dir}/test_cases_record.md")
    # print(f"[INFO] Restored test_cases_record.md to sandbox from last iteration")

    # # Load fuzzer template from last iteration
    # assert iter_num > 0, f"Last iteration not found"
    # last_fuzzer_template_dir = f"{output_dir}/test_checkpoints/test_iter_{iter_num - 1}/fuzzer_template.py"
    # assert os.path.exists(last_fuzzer_template_dir), f"Last iteration fuzzer template not found"
    # # Copy fuzzer template back to sandbox
    # shutil.copy(f"{last_fuzzer_template_dir}", f"{sandbox_dir}/fuzzer_template.py")
    # print(f"[INFO] Restored fuzzer_template.py to sandbox from last iteration")


def _save_test_cases(work_dir: str):
    """Save current memory and test cases for future recovery."""
    # Ensure test checkpoints directory exists and is up to date
    test_cases_dir = f"{work_dir}/test_cases/"
    assert os.path.exists(test_cases_dir), f"Test cases not found"
    
    # Copy current test cases from sandbox to checkpoint
    sandbox_dir = f"{work_dir}/sandbox/"
    _copy_directory(sandbox_dir, test_cases_dir, whitelist=white_list_for_copy_test_cases)
    
    # # save the test_cases_record.md
    # assert os.path.exists(f"{sandbox_dir}/test_cases_record.md"), f"Test cases record not found"
    # shutil.copy(f"{sandbox_dir}/test_cases_record.md", f"{test_checkpoint_dir}/test_cases_record.md")

    # # save the current fuzzer_template.py
    # assert os.path.exists(f"{sandbox_dir}/fuzzer_template.py"), f"Fuzzer template not found"
    # shutil.copy(f"{sandbox_dir}/fuzzer_template.py", f"{test_checkpoint_dir}/fuzzer_template.py")
        

def discriminator(proj_name: str, project_instance: str, work_dir: str, iter_num: int, is_bsd: bool = False):
    if DEBUG: print(f"[INFO:{project_instance}] --- TestGen is running ---")
    if is_bsd:
        task_prompt = CHECKER_TASK_PROMPT_TPL_FOR_BSD
    else:
        task_prompt = CHECKER_TASK_PROMPT_TPL
    worker_num = 0
    while worker_num < 3:
        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} is running...")
        checker_agent = prepare_agent(
            "cli",
            "claude-sonnet-4-20250514",
            work_dir,
            agent_id=f"{project_instance}_discriminator_iter_{iter_num}_worker_{worker_num}"
        )
        run_agent(
            checker_agent,
            task_prompt.replace("<version>", f"{worker_num}").replace("<current_test_cases_number>",f"{SEED_TEST_CASES + (iter_num - 1) * TEST_CASES_PER_ITERATION}").replace("<TEST_CASES_PER_ITERATION>", f"{TEST_CASES_PER_ITERATION}")
        )
        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} finished running.")
        is_valid, output = validate_test(proj_name, work_dir, worker_num, is_bsd)
        if is_valid:
            if DEBUG: print(f"[INFO:{project_instance}] Validation passed.")
            break
        worker_num += 1
        
        if worker_num >= 3 and "Mismatch count validation failed" in output:
            if DEBUG: print(f"[INFO:{project_instance}] TestGen failed. Worker runs out of 3 times.")
            break        
    if DEBUG: print(f"[INFO:{project_instance}] --- TestGen finished running ---")
