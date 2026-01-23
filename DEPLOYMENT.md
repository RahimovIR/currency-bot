# Deployment Guide (fully automated Docker)

This guide explains how to deploy the Currency Bot to production fully automated using Docker and GitHub Actions.

## Prerequisites

### Server Setup

1. Install Docker on Ubuntu:
```bash
sudo apt update
sudo apt install -y docker.io docker-compose
```

2. Verify installation:
```bash
docker --version
```

### GitHub Repository Settings

Add the following secrets to your GitHub repository (`Settings > Secrets and variables > Actions`):

| Secret Name | Description | Example |
|-------------|-------------|---------|
| `SERVER_HOST` | Server IP or domain | `192.168.1.100` or `example.com` |
| `SERVER_USER` | SSH username | `ubuntu` |
| `SSH_PRIVATE_KEY` | Private SSH key for server access | `-----BEGIN OPENSSH PRIVATE KEY-----...` |
| `TELOXIDE_TOKEN` | Telegram bot token | `1234567890:ABC...` |
| `GHCR_TOKEN` | GitHub Personal Access Token (GHCR scopes) | `ghp_...` |

### Generate SSH Keys

1. Generate SSH key pair (if you don't have one):
```bash
ssh-keygen -t ed25519 -C "github-actions"
```

2. Copy public key to server:
```bash
ssh-copy-id ubuntu@your-server
```

3. Add private key to GitHub Secrets:
```bash
cat ~/.ssh/id_ed25519
```

Copy the entire output and add it as `SSH_PRIVATE_KEY` secret.

### Configure SSH Alias

Add to your `~/.ssh/config`:
```
Host vpn
    HostName your-server-ip
    User ubuntu
    IdentityFile ~/.ssh/id_ed25519
```

Now you can connect with `ssh vpn`.

## Deployment

### Option 1: Automatic Deployment (Recommended)

Push to `master` branch triggers automatic deployment:
```bash
git add .
git commit -m "Update bot"
git push origin master
```

Or manually trigger via GitHub UI: `Actions > Deploy > Run workflow`

### Option 2: Manual Deployment

1. Build Docker image:
```bash
docker build -t currency-bot:latest .
```

2. Push to GHCR:
```bash
docker tag currency-bot:latest ghcr.io/YOUR_USERNAME/currency-bot:latest
docker push ghcr.io/YOUR_USERNAME/currency-bot:latest
```

3. Deploy to server:
```bash
ssh vpn
docker pull ghcr.io/YOUR_USERNAME/currency-bot:latest
docker run -d --name currency-bot --restart unless-stopped -e TELOXIDE_TOKEN="your-token" ghcr.io/YOUR_USERNAME/currency-bot:latest
```

## Server Management

### Check Container Status
```bash
ssh vpn "docker ps -a | grep currency-bot"
```

### View Logs
```bash
ssh vpn "docker logs -f currency-bot"
```

### Restart Container
```bash
ssh vpn "docker restart currency-bot"
```

### Stop Container
```bash
ssh vpn "docker stop currency-bot"
```

### Update Container
```bash
ssh vpn "docker pull ghcr.io/YOUR_USERNAME/currency-bot:latest && docker stop currency-bot && docker rm currency-bot && docker run -d --name currency-bot --restart unless-stopped -e TELOXIDE_TOKEN='your-token' ghcr.io/YOUR_USERNAME/currency-bot:latest"
```

## Systemd Service (Optional)

To manage container with systemd:

1. Create service file on server:
```bash
ssh vpn "sudo tee /etc/systemd/system/currency-bot.service << EOF
[Unit]
Description=Currency Bot
After=docker.service
Requires=docker.service

[Service]
Restart=always
ExecStart=/usr/bin/docker start currency-bot
ExecStop=/usr/bin/docker stop -t 10 currency-bot

[Install]
WantedBy=multi-user.target
EOF"
```

2. Enable and start service:
```bash
ssh vpn "sudo systemctl daemon-reload && sudo systemctl enable --now currency-bot"
```

3. Check service status:
```bash
ssh vpn "sudo systemctl status currency-bot"
```

## Troubleshooting

### Container won't start
```bash
ssh vpn "docker logs currency-bot"
```

### SSH connection fails
- Verify `SSH_PRIVATE_KEY` is correct (no trailing spaces)
- Check server firewall allows SSH (port 22)
- Verify `SERVER_HOST` and `SERVER_USER` secrets

### Docker login fails
- Verify `GHCR_TOKEN` has correct scopes
- Ensure GitHub repository visibility allows GHCR access

### Bot not responding
- Check `TELOXIDE_TOKEN` is correct
- Verify container is running: `ssh vpn "docker ps"`
- View logs: `ssh vpn "docker logs -f currency-bot"`
