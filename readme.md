# powerctl

A simple command-line (and optional GUI) power management tool for Linux, built with Rust.

## Features

- Shutdown, reboot, or suspend your system from the terminal
- Optional GUI confirmation dialog (via [egui](https://github.com/emilk/egui))
- Interactive confirmation prompt to prevent accidental actions

## Requirements

- Linux with `systemctl` available
- Rust toolchain (to build from source)

## Installation

```bash
git clone https://github.com/yok1rai/powerctl
cd powerctl
cargo build --release
```

The compiled binary will be at `target/release/powerctl`. You can move it to your PATH:

```bash
cargo install --path .
```

## Usage

```
powerctl <option>
```

| Full flag    | Short | Description                  |
|--------------|-------|------------------------------|
| `--shutdown` | `-sd` | Shut down the computer       |
| `--reboot`   | `-r`  | Reboot the computer          |
| `--sleep`    | `-sl` | Put the computer to sleep    |
| `--help`     | `-h`  | Show this help message       |

### Examples

```bash
powerctl --shutdown   # prompts for confirmation, then shuts down
powerctl -r           # prompts for confirmation, then reboots
powerctl --help       # prints the help message
```

When you run a power action, you'll be asked to confirm:

```
Do you want to continue? yes
```

Type `yes` / `y` to proceed, or `no` / `n` to cancel.

## GUI Mode

Set the `GUI` environment variable to launch a graphical confirmation dialog instead:

```bash
GUI=1 powerctl --shutdown
```

> **Note:** The `--help` flag is not available in GUI mode.

## License

MIT
