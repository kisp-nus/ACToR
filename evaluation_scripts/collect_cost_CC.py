#!/usr/bin/env python3

import json
import os
import glob
from pathlib import Path
from collections import defaultdict
import re


def find_latest_files(target_name, archive_dir, model_usage_v, iteration_num = 10):
    """Find the latest timestamp files for a given target."""
    files = []
    
    for file_type in ['translator', 'discriminator']:
        pattern = f"{target_name}_{file_type}_*/*.stdout"
        if iteration_num == 0:
            regex = re.compile(r".*_iter_0_worker_[0-2]_.*\.stdout$")
        elif iteration_num == 10:
            regex = re.compile(r".*_iter_(10|[0-9])_worker_[0-2]_.*\.stdout$")
        elif iteration_num == 20:
            regex = re.compile(r".*_iter_(20|1[0-9]|[0-9])_worker_[0-2]_.*\.stdout$")
        elif iteration_num == 25:
            regex = re.compile(r".*_iter_(25|2[0-4]|1[0-9]|[0-9])_worker_[0-2]_.*\.stdout$")
        else:
            assert False, f"Iteration number {iteration_num} not supported"

        _files = glob.glob(os.path.join(archive_dir, pattern))
        matching_files = [f for f in _files if regex.search(f)]

        # if matching_files == []:
            # assert False, f"No files found for {target_name}_{file_type}"
        
        files.extend(matching_files)
 
    return files

def parse_cost_data(file_path, model_usage_v):
    """Parse cost data from a stdout file."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Find the last non-empty line that looks like JSON
        last_line = None
        for line in reversed(lines):
            line = line.strip()
            if line and (line.startswith('{') or line.startswith('[')):
                last_line = line
                break
        
        if not last_line:
            print(f"  Warning: No JSON-like line found in {file_path}")
            return None
            
        # Parse JSON
        try:
            data = json.loads(last_line)
        except json.JSONDecodeError as e:
            print(f"  Warning: JSON decode error in {file_path}: {e}")
            return None
            
        # Check if it's a result type
        if data.get("type") != "result":
            print(f"  Warning: Last line is not result type in {file_path}, got: {data.get('type')}")
            return None
            
        # Extract cost and token information
        usage = data.get("usage", {})
        
        # Check if usage tokens are all zero (indicating failed session)
        usage_tokens_sum = (usage.get("input_tokens", 0) + 
                           usage.get("cache_creation_input_tokens", 0) + 
                           usage.get("cache_read_input_tokens", 0) + 
                           usage.get("output_tokens", 0))
        
        # If usage tokens are zero but we have cost, try to get tokens from modelUsage
        if usage_tokens_sum == 0 and data.get("total_cost_usd", 0) > 0:
            model_usage = data.get("modelUsage", {})
            
            # Only collect tokens from the specified model version
            if model_usage_v in model_usage:
                model_data = model_usage[model_usage_v]
                total_input_tokens = model_data.get("inputTokens", 0)
                total_output_tokens = model_data.get("outputTokens", 0)
                total_cache_read_tokens = model_data.get("cacheReadInputTokens", 0)
                total_cache_creation_tokens = model_data.get("cacheCreationInputTokens", 0)
                
                cost_data = {
                    "total_cost_usd": data.get("total_cost_usd", 0),
                    "input_tokens": total_input_tokens,
                    "cache_creation_input_tokens": total_cache_creation_tokens,
                    "cache_read_input_tokens": total_cache_read_tokens,
                    "output_tokens": total_output_tokens,
                    "data_source": "modelUsage"  # Flag to indicate source
                }
                
                print(f"  Warning: Used modelUsage data for {model_usage_v} (session likely failed)")
            else:
                # If specified model not found, fall back to zeros but keep cost
                cost_data = {
                    "total_cost_usd": data.get("total_cost_usd", 0),
                    "input_tokens": 0,
                    "cache_creation_input_tokens": 0,
                    "cache_read_input_tokens": 0,
                    "output_tokens": 0,
                    "data_source": "modelUsage_missing"  # Flag to indicate missing model data
                }
                
                print(f"  Warning: {model_usage_v} not found in modelUsage, using zeros for tokens")
        else:
            # Use regular usage data
            cost_data = {
                "total_cost_usd": data.get("total_cost_usd", 0),
                "input_tokens": usage.get("input_tokens", 0),
                "cache_creation_input_tokens": usage.get("cache_creation_input_tokens", 0),
                "cache_read_input_tokens": usage.get("cache_read_input_tokens", 0),
                "output_tokens": usage.get("output_tokens", 0),
                "data_source": "usage"  # Flag to indicate source
            }
        
        return cost_data
        
    except Exception as e:
        print(f"  Error parsing {file_path}: {e}")
        return None

def collect_cost_cc(instances, model_usage_v, iteration_num = 10):
    # Get the archive directory
    script_dir = Path(__file__).parent
    
    archive_dir = script_dir.parent / "__utils" / "_lproc" / ".lproc_archive"
    
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
        files = find_latest_files(target_name, archive_dir, model_usage_v, iteration_num)
        files = list(set(files))

        total_files += len(files)
        if not files:
            print(f"  No files found for {target_name}")
            continue

        # for file in files:
        #     print(file)
            
        for file_path in files:
            cost_data = parse_cost_data(file_path, model_usage_v)
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
    print(f"Total input tokens: {totals['input_tokens']:,}")
    print(f"Total cache creation tokens: {totals['cache_creation_input_tokens']:,}")
    print(f"Total cache read tokens: {totals['cache_read_input_tokens']:,}")
    print(f"Total output tokens: {totals['output_tokens']:,}")
    
    print("\nOverall Averages:")
    print(f"Average cost per entry: ${totals['total_cost_usd']/num_entries:.6f}")
    print(f"Average input tokens per entry: {totals['input_tokens']/num_entries:.1f}")
    print(f"Average cache creation tokens per entry: {totals['cache_creation_input_tokens']/num_entries:.1f}")
    print(f"Average cache read tokens per entry: {totals['cache_read_input_tokens']/num_entries:.1f}")
    print(f"Average output tokens per entry: {totals['output_tokens']/num_entries:.1f}")

