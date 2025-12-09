import json
import os
import shutil
import subprocess
import random
from agents.cli_agent import MyCLIAgent
from agents.utils.local_env import LocalEnvironment
from agents.utils.models import AnthropicModel, OpenAIModel
import time
from pathlib import Path
from utils import white_list_for_copy_c, white_list_for_copy_rs, white_list_for_copy_test_cases, _copy_directory, run_cc

DEBUG = False
TRANSLATION_TASK_PROMPT_TPL = f"""
<task>
You are an expert in C and Rust.
Your task is to translate the C `<project_name>` project to safe Rust implementations.

---

## Project Setup
- The source **C code** is located in the **main folder**.  
- The test script is located in the **./testcmp.sh**. You should run `./testcmp.sh --help` to understand how to use the test script.
- The translated **Rust code** should be put in the **`ts/` folder**.  
- The compiled binary file should be put in `ts/target/release/<project_name>`.

---

## Workflow
1. Read the C code and the test script to understand the functionalities.
2. Initialize a new Cargo project in the `ts/` folder. You must use the binary name `<project_name>` in your Cargo.toml file.
3. Translate the C code to Rust code and compile it into binary.
4. Run `./testcmp.sh` to compare the output of the translated Rust code with the original C program. You should run `./testcmp.sh --help` to understand how to use the test script.
5. Clean the working directory by removing temporary files and backup files.

--- 

## Constraints
- You should double check that the Cargo.toml file uses the correct binary name `<project_name>` which is the name of the C project.
- The translated Rust code MUST compile and MUST be 100% safe. You must NOT use `unsafe`, `RefCell`, `Rc`, `Arc`, `Mutex` or FFI in your Rust code.
- The translated Rust code MUST pass all the unit tests.
- You MUST translate all the functionalities.
- You must NOT omit or simplify functionalities and test cases during translation.
- You must NOT modify the test script and test cases.
- You MUST only work in current `sandbox/` folder, don't touch any files outside.
</task>
"""

FIX_TASK_PROMPT_TPL = f"""
<task>
You are an expert in C and Rust.
Your task is to fix the buggy Rust translation of the C `<project_name>` project.

---

## Project Setup
- The source **C code** is located in the **main folder**.  
- The test script is located in the **./testcmp.sh**. You should run `./testcmp.sh --help` to understand how to use the test script.
- The translated **Rust code** is located in the **`ts/` folder**.  
- The compiled binary of the translated Rust code is put inside `ts/target/release/<project_name>`.

---

## Workflow
1. Run the test script to compare the output of the translated Rust code with the original C program to know which test cases fail.
2. Read the C code and the translated Rust code to understand the functionalities and how to fix the bugs.
3. Fix the bugs in the translated Rust code.
4. Run the test script to compare the output of the fixed Rust code with the original C program to ensure if all the test cases pass.
5. Clean the working directory by removing temporary files and backup files.

---

## Constraints
- You should double check that the Cargo.toml file uses the correct binary name `<project_name>` which is the name of the C project.
- Before making edits to the translated Rust code, you should always back up the current file as `backup.rs`. When files get corrupted, you should restore the backup file.
- You MUST ensure that the Rust code can pass all the test cases. It means that `./testcmp.sh compare ./ts/target/release/<project_name>(compiled from Rust code)` MUST show `All tests passed!`.
- You must NOT use `unsafe`, `RefCell`, `Rc`, `Arc`, `Mutex` or FFI in your Rust code.
- Do NOT omit or simplify functionalities and test cases during fixing.
- You MUST only work in current `sandbox/` folder, don't touch any files outside.
</task>
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

def validate_ts(project_name: str, work_dir: str):
    ### ================================ ANTI-CHEATING ================================
    ### All following extra steps is to avoid cheating from the translator by modifying the test cases, src files, etc.
    sandbox_dir = f"{work_dir}/sandbox/"

    ### copy all the src c files to work_dir
    c_files_dir = f"{work_dir}/c_files/"
    assert os.path.exists(c_files_dir), f"C files not found"
    _copy_directory(c_files_dir, sandbox_dir, whitelist=white_list_for_copy_c)

    ### copy all the test case files of current iteration to work_dir
    test_cases_dir = f"{work_dir}/test_cases/"
    assert os.path.exists(test_cases_dir), f"Test cases not found"
    _copy_directory(test_cases_dir, sandbox_dir, whitelist=white_list_for_copy_test_cases)

    ### build the release version for c code
    output = subprocess.run(f"make clean && make all", cwd=sandbox_dir, shell=True, capture_output=True)
    if output.returncode != 0:
        if DEBUG: print(f"[WARNING:{project_name}] Build failed. Validation failed.")
        return False, output.stdout.decode("utf-8")
    # assert os.path.exists(f"{sandbox_dir}/{project_name}.ref"), f"Reference binary not found"

    ### build the release version for rust translation
    output = subprocess.run(f"cargo clean && cargo build --release", cwd=f"{sandbox_dir}/ts/", shell=True, capture_output=True)
    if output.returncode != 0:
        if DEBUG: print(f"[WARNING:{project_name}] Build failed. Validation failed.")
        if "unclosed" in output.stdout.decode("utf-8") or "unexpected closing" in output.stdout.decode("utf-8"):
            return False, "file corrupted"
        return False, "compile error"

    ### remove all the file or folder with name `main` (to avoid possible cheating)
    target = os.path.join(sandbox_dir, "main")
    # remove if it exists, regardless of being a file or folder
    if os.path.exists(target):
        if os.path.isdir(target):
            shutil.rmtree(target) # removes directories recursively
        else:
            os.remove(target) # removes files
    ### ================================ ANTI-CHEATING ================================

    ### run the testcmp.sh
    output = subprocess.run(f"./testcmp.sh compare ./ts/target/release/{project_name}", cwd=sandbox_dir, shell=True, capture_output=True)

    if "All tests done." in output.stdout.decode("utf-8") or "All tests passed!" in output.stdout.decode("utf-8"):
        return True, output.stdout.decode("utf-8")
    else:
        return False, output.stdout.decode("utf-8")



def translator(proj_name: str, project_instance: str, work_dir: str, iter_num: int):
    if DEBUG: print(f"[INFO:{project_instance}] --- Translator is running ---")
    worker_num = 0
    while worker_num < 3:
        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} is running...")

        translator_agent = prepare_agent(
            "cli",
            "claude-sonnet-4-5-20250929",
            work_dir,
            agent_id=f"{project_instance}_translator_iter_{iter_num}_worker_{worker_num}"
        )
        run_agent(
            translator_agent,
            TRANSLATION_TASK_PROMPT_TPL.replace("<project_name>", proj_name) if iter_num == 0 and worker_num == 0 else FIX_TASK_PROMPT_TPL.replace("<project_name>", proj_name)
        )

        if DEBUG: print(f"[INFO:{project_instance}] Worker {worker_num} finished running.")
        is_valid, output = validate_ts(proj_name, work_dir)
        if is_valid:
            if DEBUG: print(f"[INFO:{project_instance}] Validation passed.")
            break
        worker_num += 1

        if worker_num == 3:
            if DEBUG: print(f"[WARNING:{project_instance}] Translator failed. Worker runs out of 3 times.")
            break
    
    ### save the rs_files
    rs_files_dir = f"{work_dir}/rs_files/"
    assert os.path.exists(rs_files_dir), f"Rust files not found"
    _copy_directory(Path(work_dir) / "sandbox", Path(rs_files_dir), whitelist=white_list_for_copy_rs)

    if DEBUG: print(f"[INFO:{project_instance}] --- Translator finished running ---")
    return {
        'status': 'completed',
        'output': output
    }
