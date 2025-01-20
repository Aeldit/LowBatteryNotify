# LowBatteryNotify

Sends a notification to the user's desktop when battery percentage drops bellow `20`, `10` and `5` %

Uses the `notify-send` command (see https://gitlab.gnome.org/GNOME/libnotify)

To use this program, make it start at the launch of your session

On Hyprland for example, you would add the following line to you `hyprland.conf` file:

```conf
exec-once = path/to/LowBatteryNotify
```

