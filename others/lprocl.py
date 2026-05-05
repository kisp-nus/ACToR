#!/usr/bin/env python3
"""
A utility program to create lproc tasks from the todo list.
It's suitable for simple tasks that share the same instruction md file on different cases.
The todo cases are listed in the todo list file: `- [ ] {case_name}` is unfinished; `[x]` is finished.
The assumption is that 
- the CC working directory is the dir running this script/cmd.
- each case can be run independently. No shared state or not touch the same file.
- the prompt to CC is `Read the instruction md file and work on the {case_name} case. After finishing it, remember to check it in the todo list. `

Usage:
  lprocl -i <instruction_md_file> -t <todo_list_md_file> -p <parallel_num> [-m <model>] [-s <sand_config>]
Example:
  cd /data/data/src_refact_macro/_utils/analyze_alignment
  lprocl -i instruction.md -t todo.txt -p 4 -m claude-sonnet-4-5-20250929
  lprocl -i instruction.md -t todo.txt -p 4 -s sand2

Requirements of input files:
- The instruction md file explains the task and what arguments are expected.
- The todo list file has several lines: each line is a case, which is the expected argument by the instruction md file.
"""
import os
import sys
import json
import time
import subprocess
import argparse
import re
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Tuple, Set

def usage():
    print(__doc__)
    sys.exit(1)

def parse_todo_file(todo_file: str) -> Tuple[List[str], List[str]]:
    """Parse todo file and return (unfinished_cases, finished_cases)."""
    unfinished = []
    finished = []
    
    with open(todo_file, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            
            # Match patterns like "- [ ] case_name" or "- [x] case_name"
            match = re.match(r'-\s*\[([x ])\]\s*(.+)', line)
            if match:
                status, case_name = match.groups()
                if status == 'x':
                    finished.append(case_name.strip())
                else:
                    unfinished.append(case_name.strip())
            else:
                # If no checkbox pattern, treat as unfinished case
                if line.startswith('-'):
                    unfinished.append(line[1:].strip())
                else:
                    unfinished.append(line.strip())
    
    return unfinished, finished

def update_todo_file(todo_file: str, finished_cases: Set[str]):
    """Update todo file to mark finished cases."""
    lines = []
    
    with open(todo_file, 'r') as f:
        for line in f:
            original_line = line
            line_stripped = line.strip()
            
            if not line_stripped:
                lines.append(original_line)
                continue
            
            # Check if this is a todo item
            match = re.match(r'-\s*\[([x ])\]\s*(.+)', line_stripped)
            if match:
                status, case_name = match.groups()
                case_name = case_name.strip()
                if case_name in finished_cases:
                    lines.append(f"- [x] {case_name}\n")
                else:
                    lines.append(original_line)
            else:
                # Handle lines without checkbox format
                if line_stripped.startswith('-'):
                    case_name = line_stripped[1:].strip()
                else:
                    case_name = line_stripped.strip()
                
                if case_name in finished_cases:
                    lines.append(f"- [x] {case_name}\n")
                else:
                    lines.append(original_line)
    
    with open(todo_file, 'w') as f:
        f.writelines(lines)

def run_cc(cc_name: str, instr_file: str, todo_file: str, case_name: str, model: str, sand_config: str) -> bool:
    """Run CC on a single case. Returns True if successful."""
    try:
        # Clean the cc process
        subprocess.run(f"lproc -k {cc_name}", shell=True, capture_output=True)
        subprocess.run(f"lproc -d {cc_name}", shell=True, capture_output=True)

        # Init the cc process
        output = subprocess.run(
            f"lproc -s {cc_name} [proxies/claudix-sandv2.py::{sand_config}]",
            cwd=os.getcwd(),
            shell=True,
            capture_output=True
        )
        
        if output.returncode != 0:
            print(f"[ERROR] Failed to init CC process for {cc_name}")
            return False
        
        if "LProc started successfully!" not in output.stdout.decode("utf-8"):
            print(f"[ERROR] Failed to init CC process for {cc_name}")
            return False
        
        # Prepare the task prompt
        task_prompt = f"Read the {instr_file} file and work on the {case_name} case in the {todo_file}. After finishing it, remember to check it in the todo list."
        
        # Append task to stdin
        cc_stdin_path = f"/data/__utils/_lproc/.lproc/{cc_name}.stdin"
        with open(cc_stdin_path, "a") as f:
            input_msg = {
                "type": "user",
                "message": {
                    "role": "user",
                    "content": [{"type": "text", "text": task_prompt}]
                }
            }
            f.write(json.dumps(input_msg) + "\n")
        
        # Wait for CC to finish
        while True:
            time.sleep(5)
            
            # Check if CC is finished
            output = subprocess.run(
                f"lproc -p {cc_name} stdout 1 un",
                shell=True,
                capture_output=True
            )
            
            if output.returncode != 0:
                print(f"[WARNING] Failed to get CC output for {cc_name}")
                continue
            
            try:
                msg_obj = json.loads(output.stdout.decode("utf-8"))
                if msg_obj["type"] == "result":
                    print(f"[INFO] CC finished for case: {case_name}")
                    break
            except Exception as e:
                # Not finished yet, continue waiting
                continue
        
        # Clean up the cc process
        subprocess.run(f"lproc -k {cc_name}", shell=True, capture_output=True)
        subprocess.run(f"lproc -d {cc_name}", shell=True, capture_output=True)
        
        return True
        
    except Exception as e:
        print(f"[ERROR] Exception in run_cc for {case_name}: {str(e)}")
        # Try to clean up
        subprocess.run(f"lproc -k {cc_name}", shell=True, capture_output=True)
        subprocess.run(f"lproc -d {cc_name}", shell=True, capture_output=True)
        return False

def worker_function(case_name: str, instr_file: str, todo_file: str, worker_id: int, model: str, sand_config: str, cc_prefix: str) -> Tuple[str, bool]:
    """Worker function for parallel execution."""
    cc_name = f"lprocl_worker_{cc_prefix}_{worker_id}_{case_name.replace('/', '_').replace(' ', '_')}"
    print(f"[INFO] Starting worker {worker_id} for case: {case_name}")
    success = run_cc(cc_name, instr_file, todo_file, case_name, model, sand_config)
    return case_name, success

def main(instr_file: str, todo_file: str, parallel_num: int, model: str, sand_config: str, cc_prefix: str):
    """Main function to process todo list with parallel CC workers."""
    # Validate input files
    if not os.path.exists(instr_file):
        print(f"[ERROR] Instruction file not found: {instr_file}")
        sys.exit(1)
    
    if not os.path.exists(todo_file):
        print(f"[ERROR] Todo file not found: {todo_file}")
        sys.exit(1)
    
    # Parse todo file
    unfinished_cases, finished_cases = parse_todo_file(todo_file)
    finished_set = set(finished_cases)
    
    print(f"[INFO] Found {len(unfinished_cases)} unfinished cases")
    print(f"[INFO] Found {len(finished_cases)} finished cases")
    print(f"[INFO] Processing with {parallel_num} parallel workers")
    
    if not unfinished_cases:
        print("[INFO] All cases are finished!")
        return
    
    # Process cases in parallel
    try:
        with ThreadPoolExecutor(max_workers=parallel_num) as executor:
            # Submit all jobs
            future_to_case = {
                executor.submit(worker_function, case, instr_file, todo_file, i, model, sand_config, cc_prefix): case
                for i, case in enumerate(unfinished_cases)
            }
            
            # Process completed jobs as they finish
            for future in as_completed(future_to_case):
                case_name = future_to_case[future]
                try:
                    result_case, success = future.result()
                    
                    if success:
                        finished_set.add(result_case)
                        print(f"[SUCCESS] Completed case: {result_case}")
                        # Update todo file after each completion
                        update_todo_file(todo_file, finished_set)
                    else:
                        print(f"[FAILED] Failed case: {result_case}")
                        
                except Exception as e:
                    print(f"[ERROR] Exception processing {case_name}: {str(e)}")
    
    except KeyboardInterrupt:
        print("\n[INFO] Interrupted by user. Updating todo file...")
        update_todo_file(todo_file, finished_set)
        sys.exit(1)
    
    # Final update
    update_todo_file(todo_file, finished_set)
    
    # Print summary
    completed_count = len(finished_set) - len(finished_cases)
    print(f"\n[SUMMARY] Completed {completed_count} new cases")
    print(f"[SUMMARY] Total finished: {len(finished_set)}/{len(unfinished_cases) + len(finished_cases)}")

def main_cli():
    """CLI entry point for the lprocl command."""
    parser = argparse.ArgumentParser(
        description="Create lproc tasks from a todo list",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument('-i', '--instruction', required=True, help='Instruction markdown file')
    parser.add_argument('-t', '--todo', required=True, help='Todo list file')
    parser.add_argument('-p', '--parallel', type=int, default=1, help='Number of parallel workers (default: 1)')
    parser.add_argument('-m', '--model', default="claude-opus-4-6", help='Model to use (default: claude-opus-4-5-20251101)')
    parser.add_argument('-s', '--sand', default="sand", help='Sand config name (default: sand)')
    parser.add_argument('-c', '--cc', default="cc", help='CC prefix name (default: cc; use your task name if you have one)')

    args = parser.parse_args()

    main(args.instruction, args.todo, args.parallel, args.model, args.sand, args.cc)

if __name__ == "__main__":
    main_cli()

