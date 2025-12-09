#!/usr/bin/env python3
import os, sys
from pathlib import Path
from shutil import which
import tempfile
from datetime import datetime
import json

SCRIPT_DIR = Path(__file__).resolve().parent
DEFAULT_CONFIG = os.environ.get("SAND_CONFIG", str(SCRIPT_DIR / "sand.config"))

HELP = f"""\
Usage:
  {Path(sys.argv[0]).name} [--config FILE] [--no-defaults] [--create-missing] [--] [CMD [ARG ...]]

Behavior:
  - Reads writable paths from a config file (one path per line; '#' comments allowed).
  - By default also makes $PWD and /tmp writable (disable with --no-defaults).
  - Additional writable paths can be specified via SAND_ADDITIONAL_PATH env var (comma-separated).
  - Strict by default: exits if any configured path doesn't exist.
  - With --create-missing, will create missing *directories*; missing files still cause exit.
  - If arguments are provided after '--', execute that command instead of starting bash.
  - If no command is provided, start bash with a visible "(sand)" prompt.
  - If --in-docker is set, avoid mounting new proc/dev (bind existing /proc and /dev instead).
  - If --config NAME lacks a .config suffix and path separators, resolve NAME.config next to the script.
"""

LOG_FILE = SCRIPT_DIR / "sand.log"

def log_to_file(message):
    """Append message to sand.log with timestamp."""
    try:
        with open(LOG_FILE, "a") as f:
            timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            f.write(f"[{timestamp}] {message}\n")
    except Exception as e:
        # Silently fail if logging fails - don't break sandbox functionality
        pass

def err(*a): print(*a, file=sys.stderr)


def build_bash_with_prompt():
    fd, rcpath = tempfile.mkstemp(prefix="sand_rc_", suffix=".bashrc", dir="/tmp")
    Path(rcpath).write_text(
        '[[ -f ~/.bashrc ]] && source ~/.bashrc\n'
        'export SANDBOX=1\n'
        'GREEN="\\[\\e[32m\\]"\n'
        'RESET="\\[\\e[0m\\]"\n'
        'export PS1="${GREEN}(sand)${RESET} \\u@\\h:\\w\\$ "\n'
    )
    # run interactive bash using our rcfile
    return ["bash", "--rcfile", rcpath, "-i"]


def parse_args():
    if "--" in sys.argv:
        sep = sys.argv.index("--")
        our_args = sys.argv[1:sep]
        cmd = sys.argv[sep+1:]
    else:
        our_args = sys.argv[1:]
        cmd = []

    cfg = DEFAULT_CONFIG
    cfg_is_basename = False
    add_defaults = True
    create_missing = False
    in_docker = False

    i = 0
    while i < len(our_args):
        a = our_args[i]
        if a in ("-h", "--help"):
            print(HELP); sys.exit(0)
        elif a == "--config":
            i += 1
            if i >= len(our_args): err("Missing value for --config"); sys.exit(2)
            cfg = our_args[i]
            has_sep = (os.sep in cfg) or (os.altsep and os.altsep in cfg)
            cfg_is_basename = not cfg.endswith(".config") and not has_sep
        elif a == "--no-defaults":
            add_defaults = False
        elif a == "--create-missing":
            create_missing = True
        elif a == "--in-docker":
            in_docker = True
        else:
            err(f"Unknown arg: {a}\n"); print(HELP); sys.exit(2)
        i += 1
    return cfg, cfg_is_basename, add_defaults, create_missing, in_docker, cmd

def read_config_lines(cfg_path: Path):
    if not cfg_path.exists():
        err(f"Error: config file not found: {cfg_path}")
        sys.exit(1)
    out = []
    for raw in cfg_path.read_text().splitlines():
        s = raw.strip()
        if not s or s.startswith("#"):
            continue
        out.append(s)
    return out

def resolve_path(p: str, cwd: Path) -> Path:
    expanded = os.path.expandvars(os.path.expanduser(p))
    path = Path(expanded)
    if not path.is_absolute():
        path = (cwd / path)
    return path.resolve(strict=False)


def parse_config_entry(entry: str, cwd: Path):
    host_raw, sep, sandbox_raw = entry.partition(":")
    host = host_raw.strip()
    if not host:
        err(f"Error: invalid config entry (empty host path): {entry}")
        sys.exit(1)
    sandbox = sandbox_raw.strip() if sep else host
    if not sandbox:
        err(f"Error: invalid config entry (empty sandbox path): {entry}")
        sys.exit(1)
    host_path = resolve_path(host, cwd)
    sandbox_path = resolve_path(sandbox, cwd)
    return host_path, sandbox_path

def main():
    # Log invocation: arguments and environment
    log_to_file("=" * 80)
    log_to_file(f"INVOCATION: {' '.join(sys.argv)}")
    log_to_file(f"CWD: {os.getcwd()}")
    log_to_file(f"ENVIRONMENT: {json.dumps(dict(os.environ), separators=(',', ':'))}")

    cfg, cfg_is_basename, add_defaults, create_missing, in_docker, cmd = parse_args()

    if not which("bwrap"):
        err("Error: bubblewrap (bwrap) not found in PATH.")
        sys.exit(1)
    bash = which("bash")
    if not bash:
        err("Error: bash not found in PATH.")
        sys.exit(1)

    home = Path.home()
    cwd = Path.cwd().resolve()
    if cfg_is_basename:
        cfg_path = (SCRIPT_DIR / f"{cfg}.config").resolve()
    else:
        cfg_path = Path(cfg).expanduser().resolve()

    wanted = []
    if add_defaults:
        wanted.extend([
            (cwd, cwd),
            (Path("/tmp"), Path("/tmp")),
        ])

    for line in read_config_lines(cfg_path):
        wanted.append(parse_config_entry(line, cwd))

    # Add paths from SAND_ADDITIONAL_PATH environment variable
    additional_paths = os.environ.get("SAND_ADDITIONAL_PATH", "").strip()
    if additional_paths:
        for path_entry in additional_paths.split(","):
            path_entry = path_entry.strip()
            if path_entry:
                wanted.append(parse_config_entry(path_entry, cwd))

    # Resolve + de-dup
    seen = set()
    resolved = []
    for host_path, sandbox_path in wanted:
        key = (str(host_path), str(sandbox_path))
        if key in seen:
            continue
        seen.add(key)
        resolved.append((host_path, sandbox_path))

    # Validate / optionally create dirs
    for host_path, _ in resolved:
        if host_path.exists():
            continue
        if create_missing and not host_path.suffix:
            try:
                host_path.mkdir(parents=True, exist_ok=True)
            except Exception as e:
                err(f"Error: failed to create directory {host_path}: {e}")
                sys.exit(1)
        else:
            err(f"Error: required path missing: {host_path}")
            sys.exit(1)

    # Command inside sandbox
    if cmd:
        final_cmd = cmd
    else:
        final_cmd = build_bash_with_prompt()

    # Default behavior: keep real uid/gid inside the user namespace
    argv = [
        "bwrap",
        "--unshare-user", "--uid", str(os.getuid()), "--gid", str(os.getgid()),
        "--unshare-uts", "--unshare-cgroup",
        "--die-with-parent",
        "--ro-bind", "/", "/",
    ]
    # Only unshare PID namespace when we can mount a fresh /proc. If we
    # fall back to binding the host /proc, keep the host PID namespace to
    # avoid /proc/<pid> mismatches (seen as "fatal library error, lookup self").
    if not in_docker:
        argv.insert(1, "--unshare-pid")

    # Mount /proc and /dev:
    # - Normal mode: mount fresh proc/dev inside the namespace
    # - In Docker mode: some hosts disallow mounting proc/dev; fall back to bind existing ones
    if in_docker:
        argv += [
            "--ro-bind", "/proc", "/proc",
            "--dev-bind", "/dev", "/dev",
        ]
    else:
        argv += [
            "--proc", "/proc",
            "--dev", "/dev",
        ]

    # Writable binds
    for host_path, sandbox_path in resolved:
        argv += ["--bind", str(host_path), str(sandbox_path)]

    # Pass through proxies
    for key in ["HTTP_PROXY","http_proxy","HTTPS_PROXY","https_proxy",
                "NO_PROXY","no_proxy","ALL_PROXY","all_proxy"]:
        val = os.environ.get(key)
        if val:
            argv += ["--setenv", key, val]

    # Base env + chdir + sandbox marker
    argv += [
        "--setenv", "PWD", str(cwd),
        "--setenv", "HOME", str(home),
        "--setenv", "PATH", os.environ.get("PATH",""),
        "--setenv", "SANDBOX", "1",
        "--chdir", str(cwd),
        "--",
        *final_cmd,
    ]

    # Log the bwrap command before exec
    log_to_file("BWRAP COMMAND:")
    log_to_file(f"  {' '.join(argv)}")
    log_to_file("=" * 80)

    os.execvp(argv[0], argv)

if __name__ == "__main__":
    main()
