import shutil
import fnmatch
from pathlib import Path
from typing import List
import subprocess
import time
import os
import json

white_list_for_copy_c = [
    "*.c",
    "*.h",
    "*.1", # readme-like files
    "*.6", # readme-like files
    "*.7", # readme-like files
    "*.8", # readme-like files
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

def _copy_directory(src: Path, dst: Path, whitelist: List[str] = None, max_depth: int = 5, max_file_size: int = 1024 * 1024):
        """Selectively sync whitelisted files from src to dst.
        
        First removes existing whitelisted files in dst, then copies matching files from src.
        Non-whitelisted files in dst are preserved. Empty directories are cleaned up afterward.
        
        Args:
            src: Source directory
            dst: Destination directory
            whitelist: List of filename patterns (e.g., "*.c", "Makefile"). None means copy all files.
            max_depth: Maximum directory nesting depth to traverse (default: 5)
            max_file_size: Maximum file size in bytes to copy (default: 1MB)
        """
        if isinstance(src, str):
            src = Path(src)
        if isinstance(dst, str):
            dst = Path(dst)

        dst.mkdir(parents=True, exist_ok=True)
        
        # First, clean up existing whitelisted files in destination
        def _clean_whitelisted_files(path: Path, depth: int = 0):
            """Remove files matching the whitelist from destination directory."""
            if not path.exists() or not path.is_dir():
                return
            
            if depth > max_depth:
                return
            
            for item in list(path.iterdir()):
                if item.is_dir():
                    _clean_whitelisted_files(item, depth + 1)
                elif item.is_file():
                    # Remove file if it matches whitelist
                    if whitelist is None or any(fnmatch.fnmatch(item.name, pattern) for pattern in whitelist):
                        try:
                            item.unlink()
                        except Exception as e:
                            # Skip on error
                            pass
        
        # Recursively copy only whitelisted files (up to max depth k=5)
        def _recursive_copy(src_dir: Path, dst_dir: Path, depth: int = 0):
            if depth > max_depth:
                # Ignore directories nested deeper than k=5
                return
            
            for item in src_dir.iterdir():
                src_item = item
                dst_item = dst_dir / item.name
                if item.is_dir():
                    _recursive_copy(src_item, dst_item, depth + 1)
                elif whitelist is None or any(fnmatch.fnmatch(item.name, pattern) for pattern in whitelist):
                    try:
                        file_size = src_item.stat().st_size
                        if file_size < max_file_size:  # Only copy files less than max_file_size
                            # Create parent directory only when we have a file to copy
                            dst_item.parent.mkdir(parents=True, exist_ok=True)
                            shutil.copy2(src_item, dst_item)
                    except Exception as e:
                        # Optionally log or print error, but skip file on any error
                        pass
        
        def _remove_empty_dirs(path: Path):
            """Remove empty directories recursively, bottom-up."""
            if not path.is_dir():
                return
            
            # First, recursively clean subdirectories
            for item in list(path.iterdir()):
                if item.is_dir():
                    _remove_empty_dirs(item)
            
            # Then check if current directory is empty and remove it
            # Don't remove the root dst directory itself
            if path != dst and not any(path.iterdir()):
                path.rmdir()

        # Step 1: Clean existing whitelisted files from destination
        _clean_whitelisted_files(dst)
        
        # Step 2: Copy whitelisted files from source
        _recursive_copy(src, dst)
        
        # Step 3: Clean up empty directories after copying
        _remove_empty_dirs(dst)


def sanity_check(code: str) -> tuple[bool, str]:
    """Check if the code is safe to execute."""
    if "unsafe {" in code or "unsafe{" in code:
        return False, "[ERROR] Detected `unsafe` in the code. You are not allowed to use `unsafe` code."
    if "::RefCell" in code:
        return False, "[ERROR] Detected `::RefCell` in the code. You are not allowed to use `::RefCell` in your code."
    if "::Cell" in code:
        return False, "[ERROR] Detected `::Cell` in the code. You are not allowed to use `::Cell` in your code."
    if "ffi::" in code:
        return False, "[ERROR] Detected `ffi::` in the code. You are not allowed to use `ffi::` in your code."
    if "::Rc" in code:
        return False, "[ERROR] Detected `::Rc` in the code. You are not allowed to use `::Rc` in your code."
    if "::Arc" in code:
        return False, "[ERROR] Detected `::Arc` in the code. You are not allowed to use `::Arc` in your code."
    if "::Mutex" in code:
        return False, "[ERROR] Detected `::Mutex` in the code. You are not allowed to use `::Mutex` in your code."
    return True, ""

def run_cc(work_dir: str, cc_name: str, task_prompt: str, _sanity_check: bool = True):

    ### clean the cc process
    output = subprocess.run(f"lproc -k {cc_name}", shell=True, capture_output=True)
    # assert output.returncode == 0, f"Failed to kill the cc process"
    output = subprocess.run(f"lproc -d {cc_name}", shell=True, capture_output=True)
    # assert output.returncode == 0, f"Failed to delete the cc process"

    ### init the cc process
    output = subprocess.run(f"lproc -s {cc_name} [proxies/claudix-sandv2.py::sand]", cwd=f"{work_dir}/sandbox/", shell=True, capture_output=True)
    # print(output.stderr)
    assert output.returncode == 0, f"Failed to init the cc process"
    # print(output.stdout.decode("utf-8"))
    assert "LProc started successfully!" in output.stdout.decode("utf-8"), f"Failed to init the cc process"

    ### create the log and checkpoint folder
    os.makedirs(f"{work_dir}/log_files/", exist_ok=True)

    ### append the task to stdin
    cc_stdin_path = f"/data/__utils/_lproc/.lproc/{cc_name}.stdin"
    with open(cc_stdin_path, "a") as f:
        input_msg = {"type": "user", "message": {"role": "user", "content": [{"type": "text", "text": task_prompt}]}}
        f.write(json.dumps(input_msg) + "\n")

    
    ### wait for the cc to finish. check per 5 seconds.
    while True:
        time.sleep(5)

        ### check if the cc is finished
        output = subprocess.run(f"lproc -p {cc_name} stdout 1 un", shell=True, capture_output=True)
        assert output.returncode == 0, f"Failed to get the cc output"
        
        try:
            msg_obj = json.loads(output.stdout.decode("utf-8"))
            if msg_obj["type"] == "result":
                if _sanity_check:
                    ### sanity check
                    import glob

                    rust_files = glob.glob(f"{work_dir}/sandbox/ts/src/*.rs")
                    file_errors = []
                    for rust_file_path in rust_files:
                        with open(rust_file_path, "r") as f:
                            rust_code = f.read()
                        is_safe, error_msg = sanity_check(rust_code)
                        if not is_safe:
                            file_errors.append((rust_file_path, error_msg))

                    if file_errors:
                        # Build a detailed error message
                        error_msgs = []
                        for file_path, err in file_errors:
                            error_msgs.append(f"In file `{file_path}`:\n{err}")
                        combined_msg = "[ERROR] Detected not allowed code structure(s) in the following Rust files:\n" + "\n\n".join(error_msgs)
                        
                        print("[WARNING] Detected not allowed code structure(s) in Rust sources. Ask CC to fix them.")
                        cc_stdin_path = f"/data/__utils/_lproc/.lproc/{cc_name}.stdin"
                        with open(cc_stdin_path, "a") as f:
                            input_msg = {"type": "user", "message": {"role": "user", "content": [{"type": "text", "text": combined_msg}]}}
                            f.write(json.dumps(input_msg) + "\n")
                        continue
                    else:
                        # All files passed sanity check, safe to end the task
                        break
                else:
                    # end the task
                    break
        except Exception as e:
            # TODO: add detection on if cc is alive
            continue
        
        ### update log and checkpoint
        output = subprocess.run(f"lproc -p {cc_name} stdout -1 cc", shell=True, capture_output=True)
        assert output.returncode == 0, f"Failed to get the cc output"
        log_path = f"{work_dir}/log_files/{cc_name}_output.log"
        with open(log_path, "w") as f:
            f.write(output.stdout.decode("utf-8"))
        
        msg = subprocess.run(f"lproc -i {cc_name}", shell=True, capture_output=True)
        assert msg.returncode == 0, f"Failed to get the cc input"
        msg_text = msg.stdout.decode("utf-8")
        assert "AGE_ANY_IO:" in msg_text, f"Failed to get the cc input"
        age_io_seconds = int(msg_text.split("AGE_ANY_IO:")[1].split("seconds")[0])
        if age_io_seconds > 180: ### 3 minutes
            print(f"[WARNING] The CC is taking too long to respond. Force restart and resume the task.")
            with open(cc_stdin_path, "a") as f:
                input_msg = {"type": "user", "message": {"role": "user", "content": [{"type": "text", "text": "[CLAUDIX:FORCE_RESTART_RESUME] You should continue your task."}]}}
                f.write(json.dumps(input_msg) + "\n")
            continue

    ### TASK ENDED

    ### save log and checkpoint
    output = subprocess.run(f"lproc -p {cc_name} stdout -1 cc", shell=True, capture_output=True)
    assert output.returncode == 0, f"Failed to get the cc output"
    log_path = f"{work_dir}/log_files/{cc_name}_output.log"
    with open(log_path, "w") as f:
        f.write(output.stdout.decode("utf-8"))
    
    # output = subprocess.run(f"lproc -p {cc_name} stdout -1 un", shell=True, capture_output=True)
    # assert output.returncode == 0, f"Failed to get the cc output"
    # checkpoint_path = f"{output_dir}/agent_logs/{cc_name}_checkpoint.json"
    # with open(checkpoint_path, "w") as f:
    #     f.write(json.dumps(msg_obj))

    ### kill and delete the cc process
    output = subprocess.run(f"lproc -k {cc_name}", shell=True, capture_output=True)
    assert output.returncode == 0, f"Failed to kill the cc process"
    output = subprocess.run(f"lproc -d {cc_name}", shell=True, capture_output=True)
    assert output.returncode == 0, f"Failed to delete the cc process"


if __name__ == "__main__":
    src = Path(".working/printf_d7ea02/sandbox")
    dst = Path("test_copy")
    _copy_directory(src, dst, whitelist=white_list_for_copy_rs)