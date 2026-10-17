#!/bin/bash
set -euo pipefail
exec > /var/log/userdata.log 2>&1

echo "=== Installing Docker ==="
dnf install -y docker
systemctl enable --now docker
usermod -aG docker ec2-user

echo "=== Installing Docker Compose plugin ==="
mkdir -p /usr/local/lib/docker/cli-plugins
curl -SL "https://github.com/docker/compose/releases/latest/download/docker-compose-linux-x86_64" \
  -o /usr/local/lib/docker/cli-plugins/docker-compose
chmod +x /usr/local/lib/docker/cli-plugins/docker-compose

echo "=== Authenticating to GHCR ==="
echo "${ghcr_token}" | docker login ghcr.io -u "${ghcr_username}" --password-stdin

echo "=== Writing docker-compose.prod.yml ==="
mkdir -p /opt/financial-models
cat > /opt/financial-models/docker-compose.yml <<'COMPOSE'
${compose_file}
COMPOSE

echo "=== Pulling and starting containers ==="
cd /opt/financial-models
docker compose pull
docker compose up -d

echo "=== Done ==="
