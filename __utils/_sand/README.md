# Sandbox with Bubblewrap (`sand.py`)

This repo provides a simple **filesystem sandbox** powered by [bubblewrap (bwrap)](https://github.com/containers/bubblewrap).
It lets you run a shell or command with the **entire filesystem mounted read-only**, except for a whitelist of writable paths you specify in `sand.config`.

The goal:

* Protect your system from accidental writes.
* Allow selected tools (like `claude`, compilers, caches, configs) to keep working.
* Provide a clear prompt `(sand)` so you know when you’re inside the sandbox.

---

## Features

* **Read-only by default**: every path is mounted `ro` inside the namespace.
* **Configurable writable whitelist** via `sand.config`.
* **Automatic writable defaults**: current working directory (`$PWD`) and `/tmp`.
* **Dynamic additional paths**: via `SAND_ADDITIONAL_PATH` environment variable (comma-separated).
* **Network access**: allowed (so CLI tools can talk to APIs).
* **Job control**: works normally (`fg`, `bg`, `Ctrl-Z`).
* **Colored prompt**: bash prompt inside sandbox is prefixed with green `(sand)`.
* **Strict config**: exits if required paths don't exist, unless you pass `--create-missing`.
* **Logging**: all invocations logged to `sand.log` with arguments, environment, and bwrap command.

---

## Requirements

* Linux
* [bubblewrap](https://github.com/containers/bubblewrap) (`bwrap`) installed
* Python 3.7+

On Ubuntu/Debian:

```bash
sudo apt install bubblewrap
```

---

## Files

* `sand.py` — Python launcher for the sandbox
* `sand.config` — list of writable paths (relative to `$HOME` or absolute)

---

## `sand.config` Example

```
# Writable paths for tools
~/.claude
~/.cache
~/.cargo
~/.config
~/.local
~/.npm
~/.node-gyp
~/.aws

# Single file
~/.claude.json

# Alternate mount target inside sandbox
/home/fake_${USER}0:/home/${USER}
```

Format:

* One path per line
* Supports `~` and `$ENV` expansion (host and sandbox sides)
* Use `host_path:sandbox_path` to bind a host path to a different location inside the sandbox; omit `:sandbox_path` to reuse the same path
* Lines starting with `#` are comments

---

## Usage

### 1. Start an interactive sandbox shell

```bash
./sand.py
```

This drops you into **bash** with a prompt like:

```
(sand) ubuntu@host:/path$
```

### 2. Run a command inside the sandbox

```bash
./sand.py -- claude
./sand.py -- ls -la
./sand.py -- python -c 'print("hello from sandbox")'
```

### 3. Add writable paths dynamically

```bash
SAND_ADDITIONAL_PATH="~/temp,/var/mydata" ./sand.py
```

Paths in `SAND_ADDITIONAL_PATH` are comma-separated and support:
* `~` and `$ENV` expansion
* `host_path:sandbox_path` syntax for alternate mount targets

### 4. Options

* `--config FILE` — if `FILE` ends with `.config`, load it from the given path; otherwise treat it as a basename and look for `<FILE>.config` next to `sand.py`
* `--no-defaults` — don’t auto-add `$PWD` and `/tmp` as writable
* `--create-missing` — auto-create missing directories from config (files must already exist)
* `--in-docker` — use Docker-friendly mounts (bind existing `/proc` and `/dev` instead of mounting new ones)
* `--` — everything after `--` is the command to run instead of starting bash

---

## Verification

Inside the sandbox, try:

```bash
touch /etc/should_fail || echo "ok: /etc is read-only"
touch "$PWD/ok_here"
touch ~/.cache/ok_cache
```

You should see:

* First fails (`EROFS` — read-only filesystem).
* Others succeed (if those paths are whitelisted).

---

## Tips

* The sandbox sets `SANDBOX=1` in the environment. You can use this in your dotfiles:

  ```bash
  if [[ "${SANDBOX:-}" == 1 ]]; then
    echo "Running inside sandbox"
  fi
  ```
* You can symlink files into writable directories if some program insists on writing outside.
* To extend functionality, edit `sand.config` instead of touching the script.
* Use `SAND_ADDITIONAL_PATH` for one-off writable paths without modifying config files.
* All invocations are logged to `sand.log` next to `sand.py` for debugging and auditing.

---

## Why Bubblewrap?

Bubblewrap is widely used (e.g., by Flatpak) to provide unprivileged sandboxes.
It lets us create a private **mount namespace** where we can remount parts of the filesystem as read-only without affecting the host.
