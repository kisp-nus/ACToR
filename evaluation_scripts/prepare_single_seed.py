#!/usr/bin/env python3
"""
Prepare single seed for evaluation.
For each project in projects_input folder:
  - Backup tests00.jsonl to __tests00.jsonl
  - Keep only the first line in tests00.jsonl
"""

import os
import shutil
from pathlib import Path

def main():
    # Get the path to projects_input relative to this script
    script_dir = Path(__file__).parent
    projects_input_dir = script_dir.parent / "projects_input"
    
    if not projects_input_dir.exists():
        print(f"Error: projects_input directory not found at {projects_input_dir}")
        return
    
    # Iterate through each project folder
    for project_dir in projects_input_dir.iterdir():
        if not project_dir.is_dir():
            continue
        
        tests_file = project_dir / "tests00.jsonl"
        backup_file = project_dir / "__tests00.jsonl"
        
        if not tests_file.exists():
            print(f"Warning: {tests_file} does not exist, skipping {project_dir.name}")
            continue
        
        if backup_file.exists():
            print(f"Warning: Backup already exists for {project_dir.name}, skipping")
            continue
        
        # Backup the original file
        shutil.copy2(tests_file, backup_file)
        print(f"Backed up {tests_file.name} to {backup_file.name} in {project_dir.name}")
        
        # Read the first line and overwrite the file
        with open(tests_file, 'r') as f:
            first_line = f.readline()
        
        with open(tests_file, 'w') as f:
            f.write(first_line)
        
        print(f"Kept only first line in {tests_file.name} for {project_dir.name}")
    
    print("\nDone! Use revert_back_seed.py to restore original files.")

if __name__ == "__main__":
    main()

