FROM mcr.microsoft.com/devcontainers/universal:linux

RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y python3-dev python3-pip sudo curl git htop vim build-essential m4

RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y ncdu ranger tmux rsync openssh-client openssh-server cloc

# for compiling coreutils
RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y autoconf automake autopoint bison gettext gperf m4 texinfo

# install clang
RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y clang llvm clang-format clang-tidy clang-tools clangd llvm-dev python3-venv

# install Python 3.11 from deadsnakes PPA
RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y software-properties-common && \
  add-apt-repository ppa:deadsnakes/ppa -y && \
  apt-get update && \
  apt-get install -y python3.11 python3.11-venv python3.11-dev python3.11-distutils

# install clang-14
RUN DEBIAN_FRONTEND=noninteractive apt-get update && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y clang-14 llvm-14 llvm-14-dev llvm-14-tools llvm-14-runtime


# default working dir is /data/
WORKDIR /data/

# use default user
USER 1000

RUN npm install -g @anthropic-ai/claude-code
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y