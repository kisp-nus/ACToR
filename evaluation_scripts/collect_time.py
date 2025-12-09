#!/usr/bin/env python3
import json
import os
from pathlib import Path
from datetime import datetime

def find_average_time_cost(target_name, archive_dir):
    state_file = os.path.join(archive_dir, f"{target_name}/.translation_state.json")

    state_data = json.load(open(state_file))

    backups = state_data.get("backups", [])

    # sort based on iteration number
    backups.sort(key=lambda x: x.get("iteration", 0))

    time_costs = []
    for iter_num in range(len(backups) - 1):
        # get the time gap of two iter (timestamps are ISO8601 strings)
        ts_format = "%Y-%m-%dT%H:%M:%S.%f"
        ts1 = backups[iter_num].get("timestamp")
        ts2 = backups[iter_num + 1].get("timestamp")
        if not ts1 or not ts2:
            continue
        try:
            t1 = datetime.strptime(ts1, ts_format)
            t2 = datetime.strptime(ts2, ts_format)
        except Exception as e:
            continue
        time_gap = (t2 - t1).total_seconds()
        if time_gap > 30 * 60:  # if larger than 30 minutes, skip
            continue
        time_costs.append(time_gap)

    return sum(time_costs), len(time_costs)

def collect_time_cost(instances, working_dir=None):
    # Get the archive directory
    script_dir = Path(__file__).parent
    if working_dir:
        archive_dir = script_dir.parent / working_dir
    else:
        archive_dir = script_dir.parent / ".working"
    
    if not archive_dir.exists():
        print(f"Archive directory not found: {archive_dir}")
        return
    
    # Load targets
    targets = instances
    print(f"Found {len(targets)} targets to process")
    
    total_time_cost = 0
    total_iter_in_consideration = 0

    for target_name in targets:
        print(f"Processing {target_name}...")
        
        # Find latest files for this target
        total_time_cost_per_tgt, iter_in_consideration = find_average_time_cost(target_name, archive_dir)
       
        total_time_cost += total_time_cost_per_tgt
        total_iter_in_consideration += iter_in_consideration

    
    print(f"Total time cost: {total_time_cost}")
    print(f"Total iter in consideration: {total_iter_in_consideration}")
    print(f"Average time cost per iter in consideration: {total_time_cost / total_iter_in_consideration}")


