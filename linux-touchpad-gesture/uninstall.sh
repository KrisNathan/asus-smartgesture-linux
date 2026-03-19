#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="asus-touchpad-gesture-rust.service"
SERVICE_FILE="$HOME/.config/systemd/user/$SERVICE_NAME"
RULES_TARGET="/etc/udev/rules.d/99-touchpad-gestures.rules"

echo "Removing Rust touchpad gesture user service..."

if command -v systemctl >/dev/null 2>&1; then
    systemctl --user stop "$SERVICE_NAME" 2>/dev/null || true
    systemctl --user disable "$SERVICE_NAME" 2>/dev/null || true
fi

rm -f "$SERVICE_FILE"

if command -v systemctl >/dev/null 2>&1; then
    systemctl --user daemon-reload
fi

if command -v sudo >/dev/null 2>&1; then
    if [[ -f "$RULES_TARGET" ]]; then
        sudo rm -f "$RULES_TARGET"
    fi

    if command -v udevadm >/dev/null 2>&1; then
        sudo udevadm control --reload-rules
        sudo udevadm trigger
    fi

    if getent group input >/dev/null 2>&1; then
        sudo gpasswd -d "$USER" input 2>/dev/null || true
    fi
fi

echo "Uninstall complete."
echo
echo "Log out and log back in for group membership changes to fully take effect."
