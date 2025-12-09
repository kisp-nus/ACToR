#!/usr/bin/env python3
"""
Revert back to original seeds.
For each project in projects_input folder:
  - Restore tests00.jsonl from __tests00.jsonl backup
  - Remove the backup file
"""

import os
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
        
        if not backup_file.exists():
            print(f"Warning: No backup found for {project_dir.name}, skipping")
            continue
        
        # Restore from backup
        backup_file.rename(tests_file)
        print(f"Restored {tests_file.name} from backup in {project_dir.name}")
    
    print("\nDone! Original tests00.jsonl files have been restored.")

if __name__ == "__main__":
    main()

