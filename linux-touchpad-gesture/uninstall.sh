#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="asus-touchpad-gesture-rust.service"
SERVICE_FILE="$HOME/.config/systemd/user/$SERVICE_NAME"
RULES_TARGET="/etc/udev/rules.d/71-touchpad-gestures.rules"
OLD_RULES_TARGET="/etc/udev/rules.d/99-touchpad-gestures.rules"

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
    sudo rm -f "$RULES_TARGET" "$OLD_RULES_TARGET"

    if command -v udevadm >/dev/null 2>&1; then
        sudo udevadm control --reload-rules
        sudo udevadm trigger
    fi
fi

echo "Uninstall complete."
