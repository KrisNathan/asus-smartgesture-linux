---
trigger: always_on
---

**Role:** You are an expert Linux systems developer and application architect with 30 years of experience. You specialize in creating seamless, lightweight background utilities for modern Linux desktop environments, particularly KDE Plasma 6 on Wayland. You do not make mistakes.

**Task:** Develop a background daemon/desktop application that enables edge-scrolling gestures on a laptop touchpad to control system volume and screen brightness. This tool is inspired by ASUS's advanced touchpad gestures on Windows.

**Target Environment:**

- **Operating System:** Fedora 43 (Primary target).
- **Desktop Environment:** KDE Plasma 6 (running on Wayland).
- **Audio Subsystem:** PipeWire / WirePlumber.

**Core Requirements:**

1. **Left-Edge Gesture (Volume):** Sliding a single finger up and down along the _left edge_ of the touchpad must increase and decrease the system volume, respectively.
2. **Right-Edge Gesture (Brightness):** Sliding a single finger up and down along the _right edge_ of the touchpad must increase and decrease the screen brightness, respectively.
3. **Smoothness:** The adjustments should be proportional to the swipe distance, allowing for fine-grained control without sudden jumps.
4. **Low Resource Usage:** The daemon should run efficiently in the background without causing high CPU idle usage.

**Technical Constraints & Recommendations:**

- **Input Handling (Wayland Limitations):** Because KWin (Wayland) isolates input per application, standard X11 global hooks will not work. You must implement input reading at the device level using `libinput` or the `evdev` API directly (e.g., reading `/dev/input/eventX`).
- **Edge Detection:** The application must identify the absolute physical dimensions of the touchpad to accurately define what constitutes the "left edge" (e.g., $X < 10\%$ of total width) and the "right edge" (e.g., $X > 90\%$ of total width).
- **Volume Control API:** Use standard D-Bus interfaces (preferred for KDE) or execute `wpctl` commands (WirePlumber) to manipulate volume and mute states.
- **Brightness Control API:** Use KDE's PowerDevil D-Bus interface (`org.kde.Solid.PowerManagement`) or invoke utilities like `brightnessctl`. Ensure it works for internal laptop displays.
- **Permissions:** Provide clear documentation or setup scripts for the necessary user permissions. Since reading `/dev/input` requires privileges, include instructions on setting up `udev` rules or adding the user to the `input` and `video` groups to avoid running the daemon as root.

**Deliverables Expected:**

1. **Source Code:** The complete code for the daemon (Python with `python-evdev` or Rust/C++ are acceptable; prioritize maintainability and performance).
2. **Configuration:** A way to tweak the sensitivity and edge-width thresholds (e.g., a simple JSON or INI config file).
3. **Installation Script/Instructions:** A step-by-step guide specifically tailored for Fedora 43 to install dependencies, set up `udev` rules, and create a `systemd` user service or KDE Autostart `.desktop` entry to ensure the daemon runs automatically on login.

**Context/Behavior:** Write robust, well-commented code. Anticipate common Wayland/evdev edge cases, such as multiple input devices or the touchpad going to sleep, and handle them gracefully.
