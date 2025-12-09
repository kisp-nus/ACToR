#!/usr/bin/env python3

import os
import sys
import subprocess
import argparse
import glob
import re
import signal
import time
import shutil
import json
from collections import deque
from typing import List
try:
    import yaml  # Optional; templates disabled if missing
except Exception:  # ModuleNotFoundError on systems without PyYAML
    yaml = None
import shlex
try:
    import yaml  # required for command templates
except Exception:
    print("Error: PyYAML is required. Please install with: pip install pyyaml", file=sys.stderr)
    sys.exit(1)

def get_lproc_dir():
    """Get the .lproc directory path relative to the script location"""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    lproc_dir = os.path.join(script_dir, ".lproc")
    
    # Create directory if it doesn't exist
    if not os.path.exists(lproc_dir):
        try:
            os.makedirs(lproc_dir)
            print(f"Created LProc directory: {lproc_dir}")
        except OSError as e:
            print(f"Error creating LProc directory: {e}", file=sys.stderr)
            sys.exit(1)
    
    return lproc_dir

def get_lproc_archive_dir():
    """Get the .lproc_archive directory path relative to the script location"""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    archive_dir = os.path.join(script_dir, ".lproc_archive")
    if not os.path.exists(archive_dir):
        try:
            os.makedirs(archive_dir)
            print(f"Created LProc archive directory: {archive_dir}")
        except OSError as e:
            print(f"Error creating LProc archive directory: {e}", file=sys.stderr)
            sys.exit(1)
    return archive_dir

def get_lproc_file_path(lproc_name, extension):
    """Get full path for an LProc file"""
    lproc_dir = get_lproc_dir()
    return os.path.join(lproc_dir, f"{lproc_name}.{extension}")

def check_files_exist(lproc_name):
    """Check if any of the LProc files already exist"""
    extensions = ["stdin", "stdout", "stderr"]
    existing = []
    for ext in extensions:
        filepath = get_lproc_file_path(lproc_name, ext)
        if os.path.exists(filepath):
            existing.append(filepath)
    return existing

def get_lptail_path():
    """Get the full path to lptail relative to this script"""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    lptail_path = os.path.join(script_dir, "lptail")
    if not os.path.exists(lptail_path):
        print(f"Error: lptail not found at {lptail_path}", file=sys.stderr)
        sys.exit(1)
    return lptail_path

def load_templates():
    """Load command templates from lproc.yaml"""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    yaml_path = os.path.join(script_dir, "lproc.yaml")
    
    if not os.path.exists(yaml_path):
        return {}
    
    try:
        with open(yaml_path, 'r') as f:
            config = yaml.safe_load(f)
            return config.get('templates', {}) if config else {}
    except Exception as e:
        print(f"Warning: Could not load templates from lproc.yaml: {e}", file=sys.stderr)
        return {}

def expand_command_templates(command):
    """Replace [template_name] occurrences with commands or proxies.

    - [name] resolves from lproc.yaml templates.
    - [proxies/xyz.py] resolves to running the proxy script via Python.
    """
    templates = load_templates()

    # Find all [template] patterns in the command
    pattern = r'\[([^\]]+)\]'

    script_dir = os.path.dirname(os.path.abspath(__file__))

    def replace_template(match):
        inner = match.group(1).strip()
        # Proxy support: [proxies/xyz.py] or [proxies/xyz.py::arg1::arg2]
        if inner.startswith('proxies/'):
            # Split on :: to separate proxy path from arguments
            parts = inner.split('::')
            proxy_rel = parts[0]  # e.g., "proxies/claudix-sandv2.py"
            proxy_args = parts[1:]  # e.g., ["sand1", "--verbose"]

            proxy_abs = os.path.join(script_dir, proxy_rel)
            if os.path.isfile(proxy_abs):
                # Build command with quoted proxy path and args
                cmd_parts = [shlex.quote(sys.executable), shlex.quote(proxy_abs)]
                # Add arguments (also quoted for safety)
                for arg in proxy_args:
                    cmd_parts.append(shlex.quote(arg))
                return ' '.join(cmd_parts)
            else:
                print(f"Warning: Proxy script not found: {proxy_abs}", file=sys.stderr)
                return match.group(0)
        # YAML template resolution
        if inner in templates:
            return templates[inner]
        # Keep original token if not found
        return match.group(0)

    expanded = re.sub(pattern, replace_template, command)

    # Show expansion if it occurred
    if expanded != command:
        print(f"Expanded command: {expanded}")

    return expanded

def start_lproc(lproc_name, command):
    """Start a Long-Process (LProc) with nohup"""
    lptail = get_lptail_path()
    
    # Get full paths for all files
    stdin_path = get_lproc_file_path(lproc_name, "stdin")
    stdout_path = get_lproc_file_path(lproc_name, "stdout")
    stderr_path = get_lproc_file_path(lproc_name, "stderr")
    
    # Quote all paths to handle spaces and special characters
    quoted_stdin = shlex.quote(stdin_path)
    quoted_stdout = shlex.quote(stdout_path)
    quoted_stderr = shlex.quote(stderr_path)
    quoted_lptail = shlex.quote(lptail)
    
    # Build the inner command that will be run
    # Use bash -c to properly handle the user's command with all its special characters
    inner_cmd = f"touch {quoted_stdin} && {quoted_lptail} -f {quoted_stdin} | (stdbuf -oL bash -c {shlex.quote(command)}) 1>{quoted_stdout} 2>{quoted_stderr}"
    
    # Create the full command - start a new session/group for the orchestrator bash via setsid
    # This ensures all children (lptail, subshell, pipeline commands) share the same PGID.
    full_command = f'nohup setsid bash -c {shlex.quote(inner_cmd)} > /dev/null 2> /dev/null &'
    
    # Execute the command
    try:
        subprocess.Popen(full_command, shell=True, 
                        stdout=subprocess.DEVNULL, 
                        stderr=subprocess.DEVNULL,
                        preexec_fn=os.setpgrp)  # Detach from parent process group
        return True
    except Exception as e:
        print(f"Error starting LProc: {e}", file=sys.stderr)
        return False

def run_command(cmd):
    """Run a shell command and return output"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        return result.stdout.strip()
    except Exception as e:
        return ""

def get_process_info(pid):
    """Get process name and command for a PID"""
    try:
        with open(f"/proc/{pid}/comm", 'r') as f:
            name = f.read().strip()
        with open(f"/proc/{pid}/cmdline", 'r') as f:
            cmdline = f.read().replace('\0', ' ').strip()
        return name, cmdline if cmdline else name
    except:
        return None, None

def _read_proc_cmdline_tokens(pid):
    """Return argv tokens from /proc/<pid>/cmdline as a list, or []."""
    try:
        with open(f"/proc/{pid}/cmdline", 'rb') as f:
            data = f.read()
        if not data:
            return []
        parts = data.split(b'\0')
        parts = [p.decode(errors='ignore') for p in parts if p]
        return parts
    except Exception:
        return []

def _read_proc_ppid(pid):
    """Return parent PID by reading /proc/<pid>/status (PPid), or None."""
    try:
        with open(f"/proc/{pid}/status", 'r') as f:
            for line in f:
                if line.startswith('PPid:'):
                    try:
                        return int(line.split(':', 1)[1].strip())
                    except Exception:
                        return None
    except Exception:
        return None

def _read_children_pids(pid):
    """Return list of child PIDs via /proc/<pid>/task/<pid>/children."""
    try:
        with open(f"/proc/{pid}/task/{pid}/children", 'r') as f:
            content = f.read().strip()
        if not content:
            return []
        return [int(x) for x in content.split() if x.isdigit()]
    except Exception:
        return []

def _scan_lptail_index():
    """Scan /proc once and build an index of stdin_file -> lproc_info.

    Fast and robust: avoids lsof/ps. Matches processes where argv[0] basename is
    'lptail' and argv contains '-f', '<stdin path>'.
    """
    index = {}
    lproc_dir = get_lproc_dir()
    # Iterate numeric PIDs in /proc
    for entry in os.listdir('/proc'):
        if not entry.isdigit():
            continue
        pid = int(entry)
        argv = _read_proc_cmdline_tokens(pid)
        if not argv:
            continue
        exe_base = os.path.basename(argv[0])
        if exe_base != 'lptail':
            continue
        # Find '-f' and the following token as the stdin path
        try:
            fi = argv.index('-f')
            stdin_path = argv[fi + 1] if fi + 1 < len(argv) else None
        except ValueError:
            stdin_path = None
        if not stdin_path:
            continue
        # Only consider stdin files under our managed .lproc directory
        if not stdin_path.startswith(lproc_dir + os.sep) or not stdin_path.endswith('.stdin'):
            continue

        # Parent bash orchestrator
        bash_pid = _read_proc_ppid(pid)
        if not bash_pid:
            continue

        # Siblings of lptail under the same parent (likely one: the subshell)
        siblings = [c for c in _read_children_pids(bash_pid) if c != pid]
        subshell_pid = None
        chosen = None
        for s in siblings:
            _name, scmd = get_process_info(s)
            if scmd and 'stdbuf' in scmd:
                chosen = s
                break
        subshell_pid = chosen if chosen is not None else (siblings[0] if siblings else None)

        # Locate the inner bash -c (receives stdin from lptail via stdbuf)
        inner_bash_pid = None
        inner_bash_c_payload = None
        if subshell_pid:
            # Case 1: subshell itself is the inner bash -c
            argv_sub = _read_proc_cmdline_tokens(subshell_pid)
            if argv_sub and os.path.basename(argv_sub[0]) == 'bash':
                try:
                    ci = argv_sub.index('-c')
                    if ci + 1 < len(argv_sub):
                        inner_bash_pid = subshell_pid
                        inner_bash_c_payload = argv_sub[ci + 1]
                except ValueError:
                    pass

            level1 = _read_children_pids(subshell_pid)
            # First, look directly under subshell for bash -c
            for c1 in level1:
                argv1 = _read_proc_cmdline_tokens(c1)
                if argv1 and os.path.basename(argv1[0]) == 'bash':
                    try:
                        ci = argv1.index('-c')
                        if ci + 1 < len(argv1):
                            inner_bash_pid = c1
                            inner_bash_c_payload = argv1[ci + 1]
                            break
                    except ValueError:
                        pass
            # If not found, inspect stdbuf child, then its bash
            if inner_bash_pid is None:
                for c1 in level1:
                    argv1 = _read_proc_cmdline_tokens(c1)
                    if argv1 and os.path.basename(argv1[0]) == 'stdbuf':
                        level2 = _read_children_pids(c1)
                        for c2 in level2:
                            argv2 = _read_proc_cmdline_tokens(c2)
                            if argv2 and os.path.basename(argv2[0]) == 'bash':
                                try:
                                    ci = argv2.index('-c')
                                    if ci + 1 < len(argv2):
                                        inner_bash_pid = c2
                                        inner_bash_c_payload = argv2[ci + 1]
                                        break
                                except ValueError:
                                    pass
                        if inner_bash_pid is not None:
                            break

        # Resolve command processes robustly:
        # - If an inner bash -c exists, its children are commands. However, bash may exec the final
        #   command (PID reused) and have no children; in that case, treat the subshell itself as the
        #   command if it isn't a bash/stdbuf wrapper.
        # - If no inner bash is found, and the subshell exists and isn't a bash/stdbuf wrapper, treat
        #   the subshell itself as the command. Otherwise fall back to subshell children.
        command_pids = []
        subshell_argv0 = None
        if subshell_pid:
            argv_sub = _read_proc_cmdline_tokens(subshell_pid)
            subshell_argv0 = os.path.basename(argv_sub[0]) if argv_sub else None
        if inner_bash_pid:
            command_pids = _read_children_pids(inner_bash_pid)
            if not command_pids and subshell_pid and subshell_argv0 not in ("bash", "stdbuf"):
                command_pids = [subshell_pid]
        else:
            if subshell_pid and subshell_argv0 not in ("bash", "stdbuf"):
                command_pids = [subshell_pid]
            else:
                command_pids = _read_children_pids(subshell_pid) if subshell_pid else []

        lproc_name = os.path.basename(stdin_path)[:-6]  # strip .stdin

        # Gather bash process details once and derive PGID and bash -c payload
        b_name, b_cmd = get_process_info(bash_pid)
        try:
            pgid_val = os.getpgid(bash_pid)
        except Exception:
            pgid_val = None
        # Prefer argv tokens to extract the exact -c payload
        bash_argv = _read_proc_cmdline_tokens(bash_pid)
        bash_c_payload = None
        if bash_argv:
            try:
                ci = bash_argv.index('-c')
                if ci + 1 < len(bash_argv):
                    bash_c_payload = bash_argv[ci + 1]
            except ValueError:
                bash_c_payload = None
        # If inner_bash_c wasn't found via process tree, try to extract it from the outer bash -c payload
        if not inner_bash_c_payload and bash_c_payload:
            try:
                m = re.search(r"bash\s+-c\s+(['\"])(.*?)\1", bash_c_payload)
                if m:
                    inner_bash_c_payload = m.group(2)
            except Exception:
                pass

        info = {
            'name': lproc_name,
            'stdin_file': stdin_path,
            'stdout_file': get_lproc_file_path(lproc_name, "stdout"),
            'stderr_file': get_lproc_file_path(lproc_name, "stderr"),
            'tail_process': {
                'pid': pid,
                'name': get_process_info(pid)[0] or 'lptail',
                'cmd': ' '.join(argv)
            },
            'bash_process': {
                'pid': bash_pid,
                'name': b_name or 'bash',
                'cmd': b_cmd or ''
            },
            'pgid': pgid_val,
            'bash_c': bash_c_payload or '',
            'inner_bash': {
                'pid': inner_bash_pid,
                'cmd': inner_bash_c_payload or ''
            },
            'inner_bash_c': inner_bash_c_payload or '',
            'command_processes': []
        }
        for cp in command_pids:
            n, c = get_process_info(cp)
            if n:
                info['command_processes'].append({'pid': cp, 'name': n, 'cmd': c})

        index[stdin_path] = info
    return index

def get_parent_pid(pid):
    """Get parent PID of a process"""
    try:
        output = run_command(f"ps -o ppid= -p {pid}")
        if output:
            return int(output.strip())
    except:
        pass
    return None

def get_sibling_processes(ppid, exclude_pid):
    """Get all sibling processes (other children of the same parent)"""
    siblings = []
    try:
        output = run_command(f"ps --ppid {ppid} -o pid=")
        if output:
            for pid_str in output.split():
                pid = int(pid_str)
                if pid != exclude_pid:
                    siblings.append(pid)
    except:
        pass
    return siblings

def find_lproc_processes(stdin_file):
    """Find all processes related to an LProc by its stdin file using /proc scan."""
    index = _scan_lptail_index()
    return index.get(stdin_file)

def get_lprocs_data():
    """Get structured data about all LProcs (for programmatic use)"""
    # Find all .stdin files in the .lproc directory
    lproc_dir = get_lproc_dir()
    stdin_pattern = os.path.join(lproc_dir, "*.stdin")
    stdin_files = glob.glob(stdin_pattern)
    
    if not stdin_files:
        return [], [], lproc_dir
    
    # Sort stdin files by file change time (ctime on Linux; oldest first)
    stdin_files.sort(key=lambda f: os.path.getctime(f))
    
    # Build a single-pass index of running lprocs
    index = _scan_lptail_index()
    
    running_lprocs = []
    inactive_lprocs = []
    
    for stdin_file in stdin_files:
        lproc_info = index.get(stdin_file)
        if lproc_info:
            # Add creation time for display
            lproc_info['ctime'] = os.path.getctime(stdin_file)
            running_lprocs.append(lproc_info)
        else:
            # Extract just the name from full path
            inactive_lprocs.append(os.path.basename(stdin_file)[:-6])  # Remove .stdin
    
    return running_lprocs, inactive_lprocs, lproc_dir

def print_process_group(lproc_info):
    """Print the process group information for an LProc (reusable for -l and -i)"""
    print("Process Group:")

    # LPTail process
    tail_cmd = lproc_info['tail_process']['cmd'][:50] if len(lproc_info['tail_process']['cmd']) > 50 else lproc_info['tail_process']['cmd']
    print(f"  LPTAIL  / PID: {lproc_info['tail_process']['pid']:6} / {tail_cmd}")

    # Bash process
    bash_cmd = lproc_info['bash_process']['cmd'][:50] if len(lproc_info['bash_process']['cmd']) > 50 else lproc_info['bash_process']['cmd']
    print(f"  BASH    / PID: {lproc_info['bash_process']['pid']:6} / {bash_cmd}")

    # Command process(es)
    if lproc_info['command_processes']:
        for proc in lproc_info['command_processes']:
            cmd = proc['cmd'][:50] if len(proc['cmd']) > 50 else proc['cmd']
            print(f"  COMMAND / PID: {proc['pid']:6} / {cmd}")
    else:
        # Show inner bash -c if present as a signal the pipeline is up
        inner = lproc_info.get('inner_bash') or {}
        if inner.get('pid'):
            payload = inner.get('cmd') or ''
            short = payload[:50] + ('…' if len(payload) > 50 else '') if payload else '(bash -c)'
            print(f"  COMMAND / PID: {inner['pid']:6} / {short}")
        else:
            print(f"  COMMAND / PID: {'N/A':6} / (process exited)")

def list_lprocs():
    """List all running LProcs in the .lproc directory"""
    running_lprocs, inactive_lprocs, lproc_dir = get_lprocs_data()
    
    if not running_lprocs and not inactive_lprocs:
        print(f"No LProcs found in {lproc_dir}")
        return
    
    # Display running LProcs
    if running_lprocs:
        print("=" * 40)
        print("RUNNING LProcs (Long-Processes):")
        print("=" * 40)
        
        for i, lproc in enumerate(running_lprocs, 1):
            print(f"\n{i}. LProc: {lproc['name']}")
            print("-" * 40)

            # Display process group - one line per process
            print_process_group(lproc)

            # Display files with sizes on one line (show just the base names)
            print("\nFiles:")
            try:
                stdout_size = os.path.getsize(lproc['stdout_file'])
                stderr_size = os.path.getsize(lproc['stderr_file'])
                stdin_size = os.path.getsize(lproc['stdin_file'])
                stdin_name = os.path.basename(lproc['stdin_file'])
                stdout_name = os.path.basename(lproc['stdout_file'])
                stderr_name = os.path.basename(lproc['stderr_file'])
                print(f"  {stdin_name} ({stdin_size}B) | {stdout_name} ({stdout_size}B) | {stderr_name} ({stderr_size}B)")
            except:
                stdin_name = os.path.basename(lproc['stdin_file'])
                stdout_name = os.path.basename(lproc['stdout_file'])
                stderr_name = os.path.basename(lproc['stderr_file'])
                print(f"  {stdin_name} | {stdout_name} | {stderr_name}")
    
    # Display inactive LProcs
    if inactive_lprocs:
        print("\n" + "=" * 40)
        print("INACTIVE LProcs (files exist but processes not running):")
        print("=" * 40)
        for name in inactive_lprocs:
            print(f"  - {name}")
    
    # Summary
    print("\n" + "=" * 40)
    print(f"Summary: {len(running_lprocs)} running, {len(inactive_lprocs)} inactive")

    # JSON summary for automation scripts
    running_names = [lproc['name'] for lproc in running_lprocs]
    inactive_names = inactive_lprocs  # Already a list of names
    print(f"[SUM] AllRunningLProcs: {json.dumps(running_names)}")
    print(f"[SUM] AllInactiveLProcs: {json.dumps(inactive_names)}")

    print("=" * 40)

def _read_last_n_lines(file_path: str, n: int) -> str:
    """Return the last n lines from file as a single string.

    If n == -1, return the full file contents.
    """
    if n == -1:
        with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
            return f.read()
    if n <= 0:
        return ""
    try:
        with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
            dq = deque(maxlen=n)
            for line in f:
                dq.append(line)
        return ''.join(dq)
    except Exception as e:
        raise

def pretty_lproc(lproc_name: str, stream: str, nlines: int, converter_name: str, converter_args: List[str] = None) -> int:
    """Pretty-print last N lines of a stream using a converter.

    On success: prints converter output to stdout and returns 0.
    On failure: prints an error message to stdout and returns non-zero.
    """
    # Validate stream
    stream = stream.strip().lower()
    if stream not in ("stdin", "stdout", "stderr"):
        print(f"Error: stream must be one of stdin|stdout|stderr, got: {stream}")
        return 1

    # Resolve file path
    src_path = get_lproc_file_path(lproc_name, stream)
    if not os.path.exists(src_path):
        print(f"Error: file not found for '{lproc_name}' stream '{stream}': {src_path}")
        return 1

    # Resolve converter path
    script_dir = os.path.dirname(os.path.abspath(__file__))
    conv_filename = f"{converter_name}__{stream}_r.py"
    conv_path = os.path.join(script_dir, "converters", conv_filename)
    if not os.path.isfile(conv_path):
        print(f"Error: converter not found: {conv_path}")
        return 1

    # Read last N lines
    # Validate N (>0 or -1)
    if not (nlines == -1 or nlines > 0):
        print(f"Error: N must be a positive integer or -1 for full file, got: {nlines}")
        return 1
    try:
        data = _read_last_n_lines(src_path, nlines)
    except Exception as e:
        print(f"Error: failed reading {src_path}: {e}")
        return 1

    # Run converter via current Python
    try:
        argv = [sys.executable, conv_path]
        if converter_args:
            argv.extend(converter_args)
        proc = subprocess.run(
            argv,
            input=data,
            text=True,
            capture_output=True,
            check=False,
        )
    except Exception as e:
        print(f"Error: failed executing converter {conv_path}: {e}")
        return 1

    if proc.returncode != 0:
        # Print converter stderr or a generic message
        msg = proc.stderr.strip() if proc.stderr else f"converter exited {proc.returncode}"
        print(f"Error: {msg}")
        return proc.returncode or 1

    # Success: print exactly converter stdout (no extra characters)
    out = proc.stdout if isinstance(proc.stdout, str) else ""
    # Avoid adding extra newline; converter controls formatting
    sys.stdout.write(out)
    return 0


def append_lines_to_lproc(lproc_name: str, expected_lines: int) -> None:
    """Append exactly N lines from stdin to the target LProc stdin file."""
    if expected_lines <= 0:
        print(f"Error: N must be a positive integer, got: {expected_lines}", file=sys.stderr)
        sys.exit(1)

    try:
        raw_input = sys.stdin.read()
    except Exception as e:
        print(f"Error: failed to read stdin: {e}", file=sys.stderr)
        sys.exit(1)

    if raw_input == "":
        print("Error: no data received on stdin", file=sys.stderr)
        sys.exit(1)

    stripped = raw_input.rstrip("\r\n")
    if stripped == "":
        actual_lines = 0
        lines = []
    else:
        lines = stripped.splitlines()
        actual_lines = len(lines)

    if actual_lines != expected_lines:
        print(
            f"Error: expected {expected_lines} line(s) from stdin, but received {actual_lines}",
            file=sys.stderr,
        )
        sys.exit(1)

    stdin_path = get_lproc_file_path(lproc_name, "stdin")
    if not os.path.exists(stdin_path):
        print(
            f"Error: LProc '{lproc_name}' not found (missing stdin file at {stdin_path})",
            file=sys.stderr,
        )
        sys.exit(1)

    payload = ("\n".join(lines) + "\n") if lines else "\n"

    try:
        with open(stdin_path, "a", encoding="utf-8") as f:
            f.write(payload)
    except OSError as e:
        print(f"Error: failed to append to {stdin_path}: {e}", file=sys.stderr)
        sys.exit(1)

    print(f"Appended {expected_lines} line(s) to {stdin_path}")

def is_process_alive(pid):
    """Check if a process is still alive"""
    try:
        os.kill(pid, 0)
        return True
    except OSError:
        return False

def kill_lproc(lproc_name):
    """Kill an LProc and verify all processes are stopped"""
    stdin_file = get_lproc_file_path(lproc_name, "stdin")
    
    # Check if stdin file exists
    if not os.path.exists(stdin_file):
        print(f"Error: LProc '{lproc_name}' not found (no stdin file at {stdin_file})", file=sys.stderr)
        sys.exit(1)
    
    # Find the LProc processes
    lproc_info = find_lproc_processes(stdin_file)
    
    if not lproc_info:
        print(f"Warning: LProc '{lproc_name}' files exist but no processes are running")
        return
    
    print(f"Killing LProc '{lproc_name}'...")

    # Collect core PIDs
    tail_pid = lproc_info['tail_process']['pid']
    bash_pid = lproc_info['bash_process']['pid']
    cmd_pids = [proc['pid'] for proc in lproc_info['command_processes']]

    # Attempt group-based termination first (most robust)
    try:
        pgid = os.getpgid(bash_pid)
    except Exception:
        pgid = None

    if pgid is not None:
        try:
            print(f"  Sending SIGTERM to process group {pgid}...")
            os.killpg(pgid, signal.SIGTERM)
        except Exception as e:
            print(f"  Warning: killpg(SIGTERM, {pgid}) failed: {e}")
    else:
        # Fallback: signal individual PIDs (command processes first)
        for pid in cmd_pids:
            try:
                print(f"  Sending SIGTERM to command PID {pid}...")
                os.kill(pid, signal.SIGTERM)
            except Exception:
                pass
        for pid in (bash_pid, tail_pid):
            try:
                print(f"  Sending SIGTERM to PID {pid}...")
                os.kill(pid, signal.SIGTERM)
            except Exception:
                pass

    # Wait with short retries for graceful shutdown
    deadline = time.time() + 1.5
    while time.time() < deadline:
        alive = []
        for pid in [tail_pid, bash_pid] + cmd_pids:
            if is_process_alive(pid):
                alive.append(pid)
        if not alive:
            break
        time.sleep(0.1)

    # Escalate if needed
    remaining = []
    for pid in [tail_pid, bash_pid] + cmd_pids:
        if is_process_alive(pid):
            remaining.append(pid)

    if remaining:
        if pgid is not None:
            try:
                print(f"  Escalating: Sending SIGKILL to process group {pgid}...")
                os.killpg(pgid, signal.SIGKILL)
            except Exception as e:
                print(f"  Warning: killpg(SIGKILL, {pgid}) failed: {e}")
        # Also try individual PIDs in case they changed groups unexpectedly
        for pid in list(remaining):
            if is_process_alive(pid):
                try:
                    os.kill(pid, signal.SIGKILL)
                except Exception:
                    pass
        time.sleep(0.2)

    # Final verification
    still_alive = []
    for pid in [tail_pid, bash_pid] + cmd_pids:
        if is_process_alive(pid):
            still_alive.append(pid)

    if still_alive:
        print(f"\nError: Some processes are still alive: {still_alive}", file=sys.stderr)
        print(f"LProc '{lproc_name}' was not fully terminated!", file=sys.stderr)
        sys.exit(1)
    else:
        print(f"\nSuccess: All processes for LProc '{lproc_name}' have been terminated.")
        lproc_dir = get_lproc_dir()
        print(f"Note: The files still exist in {lproc_dir}")

def export_lproc(lproc_name, folder_path):
    """Export LProc files (stdin, stdout, stderr) to a specified folder"""
    stdin_file = get_lproc_file_path(lproc_name, "stdin")

    # Check if stdin file exists
    if not os.path.exists(stdin_file):
        print(f"Error: LProc '{lproc_name}' not found (no stdin file at {stdin_file})", file=sys.stderr)
        sys.exit(1)

    # Validate and create destination folder if needed
    try:
        dest_folder = os.path.abspath(folder_path)
        if not os.path.exists(dest_folder):
            os.makedirs(dest_folder)
            print(f"Created destination folder: {dest_folder}")
        elif not os.path.isdir(dest_folder):
            print(f"Error: '{folder_path}' exists but is not a directory", file=sys.stderr)
            sys.exit(1)
    except OSError as e:
        print(f"Error creating destination folder: {e}", file=sys.stderr)
        sys.exit(1)

    # Copy all three files
    extensions = ["stdin", "stdout", "stderr"]
    copied_files = []

    print(f"Exporting LProc '{lproc_name}' to: {dest_folder}")

    for ext in extensions:
        src_path = get_lproc_file_path(lproc_name, ext)
        if os.path.exists(src_path):
            dest_path = os.path.join(dest_folder, os.path.basename(src_path))
            try:
                shutil.copy2(src_path, dest_path)  # copy2 preserves metadata
                file_size = os.path.getsize(src_path)
                copied_files.append(ext)
                print(f"  Copied: {os.path.basename(src_path)} ({file_size} bytes)")
            except OSError as e:
                print(f"  Error copying {os.path.basename(src_path)}: {e}", file=sys.stderr)
        else:
            print(f"  Warning: {ext} file not found, skipping", file=sys.stderr)

    if copied_files:
        print(f"\nSuccessfully exported {len(copied_files)} file(s) for LProc '{lproc_name}'")
    else:
        print(f"No files were exported for LProc '{lproc_name}'", file=sys.stderr)
        sys.exit(1)

def delete_lproc(lproc_name):
    """Delete LProc files if the process is not running"""
    stdin_file = get_lproc_file_path(lproc_name, "stdin")

    # Check if stdin file exists
    if not os.path.exists(stdin_file):
        print(f"Error: LProc '{lproc_name}' not found (no stdin file at {stdin_file})", file=sys.stderr)
        sys.exit(1)

    # Check if the LProc is running
    lproc_info = find_lproc_processes(stdin_file)

    if lproc_info:
        print(f"Error: LProc '{lproc_name}' is still running!", file=sys.stderr)
        print(f"Please kill the process first with: lproc -k {lproc_name}", file=sys.stderr)
        sys.exit(1)

    # Archive all files for this LProc
    files_to_archive = []
    extensions = ["stdin", "stdout", "stderr"]

    for ext in extensions:
        filepath = get_lproc_file_path(lproc_name, ext)
        if os.path.exists(filepath):
            files_to_archive.append(filepath)

    if files_to_archive:
        archive_root = get_lproc_archive_dir()
        # Ensure a unique subdirectory per archive operation
        ts = time.strftime("%Y%m%d_%H%M%S")
        dest_dir = os.path.join(archive_root, f"{lproc_name}_{ts}")
        try:
            os.makedirs(dest_dir, exist_ok=True)
        except OSError as e:
            print(f"Error creating archive subdirectory: {e}", file=sys.stderr)
            sys.exit(1)

        print(f"Archiving files for LProc '{lproc_name}' to: {dest_dir}")
        for filepath in files_to_archive:
            try:
                dest_path = os.path.join(dest_dir, os.path.basename(filepath))
                shutil.move(filepath, dest_path)
                print(f"  Moved: {os.path.basename(filepath)}")
            except OSError as e:
                print(f"  Error moving {os.path.basename(filepath)}: {e}", file=sys.stderr)
        print(f"Successfully archived files for LProc '{lproc_name}'")
    else:
        print(f"No files found for LProc '{lproc_name}'")

def show_lproc_info(lproc_name):
    """Display detailed information about an LProc's files and processes"""
    stdin_file = get_lproc_file_path(lproc_name, "stdin")

    # Check if stdin file exists
    if not os.path.exists(stdin_file):
        print(f"Error: LProc '{lproc_name}' not found (no stdin file at {stdin_file})", file=sys.stderr)
        sys.exit(1)

    print(f"LProc Information: {lproc_name}")
    print("=" * 40)
    print()

    # Check if the lproc is running
    lproc_info = find_lproc_processes(stdin_file)

    print("Process Status:")
    if lproc_info:
        print(f"  Status: RUNNING")
        if lproc_info.get('pgid'):
            print(f"  PGID:   {lproc_info['pgid']}")
        print()
        print_process_group(lproc_info)
    else:
        print(f"  Status: NOT RUNNING")

    print()
    print("=" * 40)
    print()

    current_time = time.time()
    ages = []
    extensions = ["stdin", "stdout", "stderr"]

    for ext in extensions:
        filepath = get_lproc_file_path(lproc_name, ext)

        if os.path.exists(filepath):
            try:
                stat_info = os.stat(filepath)
                size = stat_info.st_size
                mtime = stat_info.st_mtime
                age = current_time - mtime
                ages.append(age)

                # Format timestamp
                mtime_str = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(mtime))

                print(f"{ext.upper()}: {filepath}")
                print(f"  Size:     {size} bytes")
                print(f"  Modified: {mtime_str}")
                print(f"  Age:      {int(age)} seconds")
                print()
            except OSError as e:
                print(f"{ext.upper()}: {filepath}")
                print(f"  Error: Could not stat file: {e}")
                print()
        else:
            print(f"{ext.upper()}: {filepath}")
            print(f"  Error: File not found")
            print()

    # Show AGE_ANY_IO (minimum age = most recent activity)
    if ages:
        min_age = min(ages)
        print(f"AGE_ANY_IO: {int(min_age)} seconds (most recent activity)")
    else:
        print("AGE_ANY_IO: N/A (no file data available)")

def start_command(args):
    """Handle the start command"""
    # Check if files already exist
    existing_files = check_files_exist(args.lproc_name)
    
    if existing_files:
        print(f"Error: The following files already exist:", file=sys.stderr)
        for f in existing_files:
            print(f"  - {f}", file=sys.stderr)
        print(f"\nPlease choose a different LProc name or remove existing files.", file=sys.stderr)
        sys.exit(1)
    
    # Expand command templates
    expanded_command = expand_command_templates(args.command)
    
    # Start the LProc
    print(f"Starting LProc '{args.lproc_name}' with command: {args.command}")
    
    if start_lproc(args.lproc_name, expanded_command):
        print(f"LProc started successfully!")
        print(f"\nLProc files in {get_lproc_dir()}:")
        stdin_path = get_lproc_file_path(args.lproc_name, "stdin")
        stdout_path = get_lproc_file_path(args.lproc_name, "stdout")
        stderr_path = get_lproc_file_path(args.lproc_name, "stderr")
        print(f"  Input:  {stdin_path}")
        print(f"  Output: {stdout_path}")
        print(f"  Errors: {stderr_path}")
        print(f"\nTo send data: echo 'your text' >> {stdin_path}")
        print(f"To monitor:    tail -f {stdout_path}")

        # Short readiness retry: wait briefly for lptail and subshell to attach
        ready = False
        for i in range(10):  # ~1.0–1.5s total with incremental backoff
            time.sleep(0.1 + i * 0.05)
            info = find_lproc_processes(stdin_path)
            if info:
                ready = True
                break
        if ready:
            tpid = info['tail_process']['pid'] if 'tail_process' in info else 'N/A'
            print(f"\nVerified running (lptail PID: {tpid}).")
        else:
            print("\nNote: startup may take a moment; not yet visible in process scan.")
    else:
        print(f"Failed to start LProc", file=sys.stderr)
        sys.exit(1)

def get_help_text():
    """Get help text for programmatic use"""
    help_text = """LProc - Long-Process Manager

Commands:
  -s, --start NAME COMMAND   Start a new LProc with given name and command
  -l, --list                 List all LProcs
  -k, --kill NAME            Kill an LProc by name
  -d, --delete NAME          Archive files for a stopped LProc (move to .lproc_archive)
  -e, --export NAME FOLDER   Export LProc files (stdin, stdout, stderr) to specified folder
  -a, --appendlines N NAME   Append exactly N lines from stdin to NAME.stdin (validates line count)
  -p, --pretty NAME STREAM N CONVERTER [-- ARGS...]
                          Pretty-print last N lines via converters/CONVERTER__STREAM_r.py.
                          Extra converter args must follow a literal '--' separator.
                          N: positive integer for last N lines, or -1 for full file.

Usage Examples:
  ./lproc.py -s errorlog "grep ERROR"
  ./lproc.py -s pipeline "grep WARN | sort | uniq"
  ./lproc.py -l
  ./lproc.py -k errorlog
  ./lproc.py -e errorlog /tmp/backup
  echo "{\\"msg\\": \\\"hello\\\"}" | ./lproc.py -a 1 errorlog
  ./lproc.py -p errorlog stdout 200 cc
  ./lproc.py -p errorlog stdout 200 cc -- --color

Sending Data:
  echo "test line" >> .lproc/<name>.stdin

Monitoring Output:
  tail -f .lproc/<name>.stdout

Files Created:
  .lproc/<name>.stdin   - Input file (write data here)
  .lproc/<name>.stdout  - Processed output
  .lproc/<name>.stderr  - Error messages

Process Structure:
  Each LProc consists of 3 processes:
  1. lptail - Reads the input file
  2. bash - Orchestrates the pipeline
  3. command - Your processing command

Key Features:
  • Data persists across crashes/reboots
  • Unlimited buffering with regular files
  • Multiple writers can append simultaneously
  • Easy debugging with persistent files
  • Resume capability after crashes"""
    return help_text

def main():
    parser = argparse.ArgumentParser(
        description='Manage Long-Processes (LProcs) - background processes that continuously process input',
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    # Create mutually exclusive group for commands
    group = parser.add_mutually_exclusive_group(required=True)
    
    # Start command
    group.add_argument('-s', '--start', nargs=2, metavar=('NAME', 'COMMAND'),
                      help='Start a new LProc with given name and command')
    
    # List command
    group.add_argument('-l', '--list', action='store_true',
                      help='List all LProcs in current directory')
    
    # Kill command
    group.add_argument('-k', '--kill', metavar='NAME',
                      help='Kill an LProc by name')
    
    # Delete command
    group.add_argument('-d', '--delete', metavar='NAME',
                      help='Archive files for a stopped LProc (move to .lproc_archive)')

    # Export command
    group.add_argument('-e', '--export', nargs=2, metavar=('NAME', 'FOLDER'),
                      help='Export LProc files (stdin, stdout, stderr) to specified folder')

    # Info command
    group.add_argument('-i', '--info', metavar='NAME',
                      help='Show detailed information about an LProc (size, modification time, age)')

    # Append lines command
    group.add_argument('-a', '--appendlines', nargs=2, metavar=('N', 'NAME'),
                      help='Append exactly N lines from stdin to NAME.stdin')

    # Pretty command
    group.add_argument('--pretty', '-p', nargs=argparse.REMAINDER,
                      help='Pretty-print last N lines: -p NAME STREAM N CONVERTER [-- <converter args...>]')
    
    # Use parse_known_args to allow forwarding converter args (e.g., --color) after --pretty
    args, unknown = parser.parse_known_args()
    
    if args.start:
        # Create a namespace object for start_command
        start_args = argparse.Namespace(
            lproc_name=args.start[0],
            command=args.start[1]
        )
        start_command(start_args)
    elif args.list:
        list_lprocs()
    elif args.kill:
        kill_lproc(args.kill)
    elif args.delete:
        delete_lproc(args.delete)
    elif args.export:
        export_lproc(args.export[0], args.export[1])
    elif args.info:
        show_lproc_info(args.info)
    elif args.appendlines:
        n_str, name = args.appendlines
        try:
            expected = int(n_str)
        except Exception:
            print(f"Error: N must be an integer, got: {n_str}")
            sys.exit(1)
        append_lines_to_lproc(name, expected)
    elif args.pretty is not None:
        if len(args.pretty) < 4:
            print("Error: --pretty requires at least 4 arguments: NAME STREAM N CONVERTER [-- ARGS...]")
            sys.exit(1)
        name, stream, n_str, conv, *rest = args.pretty
        # Enforce explicit '--' separator for converter args
        if rest:
            if rest[0] == '--':
                extra = rest[1:]
            else:
                print("Error: extra converter arguments must follow a '--' separator after CONVERTER")
                sys.exit(2)
        else:
            extra = []
        if unknown:
            # Accept converter args only if introduced by a top-level '--'
            if unknown[0] == '--':
                extra.extend(unknown[1:])
            else:
                print(f"Error: unrecognized arguments: {' '.join(unknown)}")
                sys.exit(2)
        try:
            n = int(n_str)
        except Exception:
            print(f"Error: N must be an integer, got: {n_str}")
            sys.exit(1)
        rc = pretty_lproc(name, stream, n, conv, extra)
        sys.exit(rc)
    else:
        # If no pretty and unknown args remain, treat as error
        if unknown:
            print(f"Error: unrecognized arguments: {' '.join(unknown)}")
            sys.exit(2)

if __name__ == "__main__":
    main()
