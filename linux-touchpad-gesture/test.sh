#!/usr/bin/env bash
set -euo pipefail

ACTION="${1:-status}"
TARGET_USER="${2:-$USER}"

find_touchpad_event() {
    local event_dir
    local name_file
    local name

    for event_dir in /sys/class/input/event*; do
        name_file="$event_dir/device/name"
        [[ -r "$name_file" ]] || continue

        name="$(<"$name_file")"
        if [[ "$name" =~ [Tt]ouchpad ]]; then
            basename "$event_dir"
            return 0
        fi
    done

    return 1
}

print_usage() {
    cat <<'EOF'
Usage:
  ./test.sh status [user]
  ./test.sh grant [user]
  ./test.sh revoke [user]

Actions:
  status  Show the detected touchpad event device and current ACLs
  grant   Temporarily grant read access to the touchpad event device
  revoke  Remove the temporary ACL for the user
EOF
}

EVENT_NAME="$(find_touchpad_event || true)"
if [[ -z "$EVENT_NAME" ]]; then
    echo "No touchpad event device found under /sys/class/input." >&2
    exit 1
fi

DEVICE_PATH="/dev/input/$EVENT_NAME"

case "$ACTION" in
    status)
        echo "Touchpad device: $DEVICE_PATH"
        echo "Touchpad name: $(<"/sys/class/input/$EVENT_NAME/device/name")"
        getfacl -p "$DEVICE_PATH"
        ;;
    grant)
        echo "Granting temporary read access on $DEVICE_PATH to user '$TARGET_USER'..."
        sudo setfacl -m "u:$TARGET_USER:r" "$DEVICE_PATH"
        getfacl -p "$DEVICE_PATH"
        ;;
    revoke)
        echo "Revoking temporary read access on $DEVICE_PATH from user '$TARGET_USER'..."
        sudo setfacl -x "u:$TARGET_USER" "$DEVICE_PATH"
        getfacl -p "$DEVICE_PATH"
        ;;
    -h|--help|help)
        print_usage
        ;;
    *)
        print_usage >&2
        exit 2
        ;;
esac
