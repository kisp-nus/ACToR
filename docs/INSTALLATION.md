# Installation Guide

This guide covers how to set up and run the ACToR system.

## Prerequisites

- Docker installed on your host machine
- At least 56GB RAM (configurable)
- Anthropic API key for Claude Code

## 1. Build and Run the Docker Container

### Build the Docker Image

Navigate to the docker directory and build the image:
```bash
cd __utils/docker
docker build -t actor-sandbox -f ./devuniv.Dockerfile .
```

The Dockerfile is based on `mcr.microsoft.com/devcontainers/universal:linux` and includes:
- Python 3.11, Clang/LLVM toolchain
- Rust toolchain (via rustup)
- Claude Code CLI (`@anthropic-ai/claude-code`)
- Build essentials for compiling C programs

### Start the Docker Container

```bash
cd __utils/docker
./start.sh
```

This script will:
- Start a container named `actor-sandbox-cont` with 56GB memory limit
- Mount the project directory to `/data/` inside the container
- Create persistent volumes for `/home/codespace`, `/usr/local`, and `/etc`

### Enter the Container Shell

```bash
./shell.sh
# Or directly:
docker exec -it actor-sandbox-cont /bin/bash
```

### Stop the Container (with Snapshot)

```bash
./stop.sh
```

## 2. Setup Inside the Container

After entering the container, run the following setup commands:

```bash
# Setup inside container
cd /data/__utils/docker/inside/
./after_start.sh
```

The script will automatically run following commmands:
```bash
set -e
# Start ssh server
sudo service ssh start

# Install bubblewrap
sudo apt install bubblewrap

# Create and activate Python virtual environment
cd /data/__utils/
python3.11 -m venv .venv
source /data/__utils/.venv/bin/activate

# Install ACToR dependencies
pip install -r /data/requirements.txt

# Install lproc
cd /data/__utils/_lproc/
pip install -e .

# Install sand
cd /data/__utils/_sand/
pip install -e .

cd /data
```

### Verify ACToR Works

```bash
python ./scripts/actor.py --help
```

## 3. Configure LLM APIs and Claude Code Account

ACToR uses Claude Code as the default coding agent. You need to configure authentication:

### Option 1: Claude Code Authentication (Interactive)

```bash
# Login to Claude Code (opens browser for OAuth)
claude login
```

This is the recommended approach for interactive use.

### Option 2: API Key File

For non-interactive or automated use, place your API key in a file:

```bash
# Create the secret directory if it doesn't exist
mkdir -p /data/./__secret__

# Save your API key to the file
echo "your-anthropic-api-key" > /data/./__secret__/claude.key

# Or if you are using GPT-5-mini
echo "your-openai-api-key" > /data/./__secret__/openai.key
```

## Configuration Summary

### API Key Options

| Method | Location | Use Case |
|--------|----------|----------|
| Claude Code login | `claude login` | Headless Mode |
| SWE Agent + Claude Models | `./__secret__/claude.key` | API Call |
| SWE Agent + GPT Models | `./__secret__/openai.key` | API Call |

## Troubleshooting

### Docker Build Fails
- Ensure you have sufficient disk space
- Check network connectivity for package downloads

### Container Won't Start
- Verify Docker daemon is running
- Check if ports are already in use
- Ensure sufficient memory is available

### Claude Code Authentication Issues
- Try `claude logout` then `claude login` again
- Verify your API key is valid
- Check network connectivity to Anthropic servers

