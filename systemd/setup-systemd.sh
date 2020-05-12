#!/bin/bash
mkdir -p ~/.config/systemd/user
cp *.service ~/.config/systemd/user

# Make sure systemd doesn't kill it after you log out
loginctl enable-linger $(whoami)

systemctl --user daemon-reload
systemctl --user enable run_site run_caddy
systemctl --user start run_site
systemctl --user start run_caddy

echo "Check logs with:"
echo "journalctl --user -u run_caddy"
echo "journalctl --user -u run_site"
