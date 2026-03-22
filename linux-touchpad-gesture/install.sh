#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_NAME="asus-touchpad-gesture-rust.service"
SERVICE_DIR="$HOME/.config/systemd/user"
SERVICE_FILE="$SERVICE_DIR/$SERVICE_NAME"
BIN_PATH="$PROJECT_DIR/target/release/linux-touchpad-gesture"
RULES_SOURCE="$PROJECT_DIR/../71-touchpad-gestures.rules"
RULES_TARGET="/etc/udev/rules.d/71-touchpad-gestures.rules"
OLD_RULES_TARGET="/etc/udev/rules.d/99-touchpad-gestures.rules"

echo "Starting Rust touchpad gesture installation..."

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: 'cargo' is required but was not found." >&2
    exit 1
fi

if ! command -v systemctl >/dev/null 2>&1; then
    echo "Error: 'systemctl' is required but was not found." >&2
    exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
    echo "Error: 'sudo' is required but was not found." >&2
    exit 1
fi

if ! command -v udevadm >/dev/null 2>&1; then
    echo "Error: 'udevadm' is required but was not found." >&2
    exit 1
fi

if [[ ! -f "$RULES_SOURCE" ]]; then
    echo "Error: udev rules file not found at $RULES_SOURCE" >&2
    exit 1
fi

if ! command -v wpctl >/dev/null 2>&1; then
    echo "Warning: 'wpctl' was not found. Volume control may not work."
fi

if ! command -v qdbus >/dev/null 2>&1; then
    echo "Warning: 'qdbus' was not found. Brightness control may not work."
fi

echo "Building release binary..."
cargo build --release --manifest-path "$PROJECT_DIR/Cargo.toml"

echo "Installing persistent input-device permissions..."
sudo rm -f "$OLD_RULES_TARGET"
sudo cp "$RULES_SOURCE" "$RULES_TARGET"
sudo udevadm control --reload-rules
sudo udevadm trigger

mkdir -p "$SERVICE_DIR"

cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=ASUS Touchpad Gesture Daemon (Rust)
After=graphical-session.target
PartOf=graphical-session.target

[Service]
Type=simple
WorkingDirectory=$PROJECT_DIR
ExecStart=$BIN_PATH
Restart=on-failure
RestartSec=2
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=read-only
ProtectControlGroups=yes
ProtectKernelLogs=yes
ProtectKernelModules=yes
ProtectKernelTunables=yes
RestrictAddressFamilies=AF_UNIX
RestrictNamespaces=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes
LockPersonality=yes
SystemCallArchitectures=native
UMask=0077

[Install]
WantedBy=graphical-session.target
EOF

systemctl --user daemon-reload
systemctl --user enable "$SERVICE_NAME"

echo
echo "Installation complete."
echo "Service file: $SERVICE_FILE"
echo
echo "Next steps:"
echo "  1. Start the user service:"
echo "     systemctl --user start $SERVICE_NAME"
echo "  2. Follow logs if needed:"
echo "     journalctl --user -u $SERVICE_NAME -f"
