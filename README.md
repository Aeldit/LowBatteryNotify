# LowBatteryNotify

Sends a notification to the user's desktop when battery percentage drops bellow `20`, `10` and `5` %

## Dependencies
Uses the `notify-send` command (see https://gitlab.gnome.org/GNOME/libnotify)

## Usage

To use this program, make it start at the launch of your session / computer

On Hyprland for example, you would add the following line to you `hyprland.conf` file:

```conf
exec-once = path/to/LowBatteryNotify
```

## Build

To build the project, use the following commands:

```sh
git clone https://github.com/Aeldit/LowBatteryNotify.git
cd LowBatteryNotify
cargo build -r # builds the project in release mode
```

The file will then be located at `target/release/LowBatteryNotify`

