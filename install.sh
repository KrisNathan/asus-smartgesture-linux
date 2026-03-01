#!/bin/bash
set -e

echo "Starting ASUS Touchpad Gesture local setup..."
PROJECT_DIR="$(pwd)"

echo "1. Checking dependencies..."
if ! command -v uv &> /dev/null; then
    echo "Error: 'uv' is required. Please install it first (e.g. 'curl -LsSf https://astral.sh/uv/install.sh | sh')."
    exit 1
fi
if ! command -v wpctl &> /dev/null; then
    echo "Warning: 'wpctl' (wireplumber) not found. Volume control may not work."
fi
if ! command -v qdbus &> /dev/null; then
    echo "Warning: 'qdbus' not found. Brightness control may not work."
fi

echo "2. Setting up Python environment with uv in $PROJECT_DIR..."
uv sync

echo "3. Creating user configuration directory..."
mkdir -p "$HOME/.config/asus-touchpad-gesture"
if [ ! -f "$HOME/.config/asus-touchpad-gesture/config.json" ]; then
    cp config.json "$HOME/.config/asus-touchpad-gesture/"
    echo "Copied default config.json to ~/.config/asus-touchpad-gesture/"
fi

echo "4. Setting up user systemd service..."
SERVICE_FILE="$HOME/.config/systemd/user/asus-touchpad-gesture.service"
mkdir -p "$HOME/.config/systemd/user"

# Dynamically create the service file with the current path
cat > "$SERVICE_FILE" << EOF
[Unit]
Description=ASUS Touchpad Gesture Daemon
After=graphical-session.target
PartOf=graphical-session.target

[Service]
Type=simple
ExecStart=$PROJECT_DIR/.venv/bin/python $PROJECT_DIR/touchpad_gesture.py
Restart=on-failure
RestartSec=3

[Install]
WantedBy=graphical-session.target
EOF

systemctl --user daemon-reload
systemctl --user enable asus-touchpad-gesture.service
# Don't start it automatically yet because the user needs input group permissions

echo ""
echo "================================================================"
echo "Local setup complete! However, to read touchpad events, you"
echo "need permission to read /dev/input/event* devices."
echo ""
echo "To do this safely without running the daemon as root, please"
echo "run the following commands manually to add the udev rule and"
echo "add yourself to the input group:"
echo ""
echo "  sudo cp 99-touchpad-gestures.rules /etc/udev/rules.d/"
echo "  sudo udevadm control --reload-rules && sudo udevadm trigger"
echo "  sudo usermod -aG input \$USER"
echo ""
echo "After doing that, LOG OUT AND LOG BACK IN for the group change"
echo "to take effect. Then start the service:"
echo "  systemctl --user start asus-touchpad-gesture.service"
echo "================================================================"
