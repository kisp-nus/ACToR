# Additional Installations
# Docker image from https://github.com/devcontainers/images/tree/main/src/universal

set -e
# start ssh server
sudo service ssh start

sudo apt install bubblewrap

cd /data/__utils/
python3.11 -m venv .venv
source /data/__utils/.venv/bin/activate

pip install -r /data/requirements.txt

cd /data/__utils/_lproc/
pip install -e .

cd /data/__utils/_sand/
pip install -e .

cd /data
