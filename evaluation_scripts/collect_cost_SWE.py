#!/usr/bin/env python3

import json
import os
import glob
from pathlib import Path
from collections import defaultdict

def find_latest_files(target_name, archive_dir, model_usage_v):
    """Find the latest timestamp files for a given target."""
    files = []
    
    for file_type in ['translator', 'discriminator']:
        pattern = f"{target_name}/log_files/cli_{model_usage_v}_{target_name}_*_checkpoint.json"

        matching_files = glob.glob(os.path.join(archive_dir, pattern))
        
        if matching_files == []:
            assert False, f"No files found for {target_name}_{file_type}"
        
        files.extend(matching_files)
 
    return files

def parse_cost_data(file_path):
    """Parse cost data from a stdout file."""
    try:
        log = json.load(open(file_path, 'r'))
        assert "total_cost" in log, f"total_cost not found in {file_path}"
        assert "total_tokens" in log, f"total_tokens not found in {file_path}"

        return {
            "total_cost_usd": log["total_cost"],
            "total_tokens": log["total_tokens"]
        }
        
    except Exception as e:
        print(f"  Error parsing {file_path}: {e}")
        return None

def collect_cost_swe(instances, model_usage_v):
    # Get the archive directory
    script_dir = Path(__file__).parent
    archive_dir = script_dir.parent / ".working"
    
    total_files = 0
    if not archive_dir.exists():
        print(f"Archive directory not found: {archive_dir}")
        return
    
    # Load targets
    targets = instances
    print(f"Found {len(targets)} targets to process")
    
    # Collect cost data
    all_cost_data = {}
    totals = defaultdict(float)
    
    for target_name in targets:
        print(f"Processing {target_name}...")
        
        # Find latest files for this target
        files = find_latest_files(target_name, archive_dir, model_usage_v)
        
        total_files += len(files)
        if not files:
            print(f"  No files found for {target_name}")
            continue
            
        for file_path in files:

                
            cost_data = parse_cost_data(file_path)
            if cost_data:
                key = f"{target_name}"
                all_cost_data[key] = cost_data
                
                # Add to totals (skip non-numeric fields)
                for metric, value in cost_data.items():
                    if metric not in ["data_source"] and isinstance(value, (int, float)):
                        totals[metric] += value
                    
                # print(f"  {file_path}: ${cost_data['total_cost_usd']:.6f}")
            else:
                pass
                # print(f"  {file_path}: Failed to parse cost data")

    # Print summary
    print("\n" + "="*60)
    print("OVERALL SUMMARY")
    print("="*60)
    
    num_entries = len(all_cost_data)
    if num_entries == 0:
        print("No cost data found!")
        return
    
    print(f"Total entries processed: {num_entries}")
    print(f"Total files processed: {total_files}")
    print(f"Total cost: ${totals['total_cost_usd']:.6f}")
    print(f"Total tokens: {totals['total_tokens']:,}")

    
    print("\nOverall Averages:")
    print(f"Average cost per entry: ${totals['total_cost_usd']/num_entries:.6f}")
    print(f"Average tokens per entry: {totals['total_tokens']/num_entries:.1f}")

