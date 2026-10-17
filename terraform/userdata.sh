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

echo "=== Fetching secrets from Secrets Manager ==="
GHCR_TOKEN=$(aws secretsmanager get-secret-value \
  --region "${aws_region}" \
  --secret-id "${ghcr_secret_arn}" \
  --query SecretString --output text)

DB_PASSWORD=$(aws secretsmanager get-secret-value \
  --region "${aws_region}" \
  --secret-id "${db_password_secret_arn}" \
  --query SecretString --output text)

echo "=== Authenticating to GHCR ==="
echo "$GHCR_TOKEN" | docker login ghcr.io -u "${ghcr_username}" --password-stdin

echo "=== Writing docker-compose.yml ==="
mkdir -p /opt/financial-models
cat > /opt/financial-models/docker-compose.yml <<'COMPOSE'
${compose_file}
COMPOSE

# Inject the real DB password
sed -i "s/__DB_PASSWORD__/$DB_PASSWORD/g" /opt/financial-models/docker-compose.yml

echo "=== Pulling and starting containers ==="
cd /opt/financial-models
docker compose pull
docker compose up -d

echo "=== Done ==="
