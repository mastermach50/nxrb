> AI written README, i'll write a proper one later, (human made code btw)

# nxrb

A simple NixOS rebuild helper written in Rust. `nxrb` wraps `nixos-rebuild` with automatic privilege elevation, optional flake updates, git commit and push on success, and build status notifications via D-Bus and [ntfy](https://ntfy.sh).

## Features

- **Automatic privilege elevation** — detects if it's not running as root and re-executes itself under `sudo` automatically
- **nixos-rebuild wrapper** — runs `nixos-rebuild switch` (default), `boot`, or `test` based on CLI flags
- **Optional flake update** — pass `--update` to run `nix flake update` before rebuilding
- **Git integration** — on a successful build, can automatically stage all changes, switch to a configured branch, and commit with a message that includes the build mode and an optional custom message
- **Git push** — optionally pushes the commit to the configured upstream branch
- **D-Bus notifications** — sends a desktop notification on build success or failure (WIP)
- **ntfy notifications** — optionally posts build status to an [ntfy](https://ntfy.sh) channel via HTTP, useful for monitoring remote builds on your phone
- **Build status summary** — prints a formatted table with build status, message, and elapsed time on exit
- **SIGINT handling** — catches Ctrl-C and runs the failure notification and status sequence cleanly before exiting
- **Auto-generated config** — on first run, writes a default `.nxrb.toml` to the current directory pre-filled with your system username and hostname

## Installation

### Via the flake (recommended)

The flake ships a Cachix binary cache so you don't need to compile from source:

```nix
# flake.nix
{
  inputs.nxrb.url = "github:mastermach50/nxrb";
}
```

Then add `inputs.nxrb.packages.${system}.default` to your environment packages.

To build and run directly:

```bash
nix run github:mastermach50/nxrb
```

### Building from source

```bash
git clone https://github.com/mastermach50/nxrb.git
cd nxrb
nix build
```

Or enter a dev shell:

```bash
nix develop
cargo build --release
```

## Configuration

On first run, `nxrb` creates a `.nxrb.toml` in the current directory and exits. Edit it before running again.

```toml
[dbus]
# The user who should receive D-Bus desktop notifications
username = "your-username"

[git]
username = "your-username"
email = "you@example.com"
commit_on_success = true
push_on_success = true
repo = "https://github.com/you/nixos-config"
branch = "main"  # can be per-device, e.g. "nixos-laptop"

[ntfy]
username = "your-username"
server = "https://ntfy.sh"
channel = "your-channel"
token = "tk_yourtoken"
icon = "https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-colours.svg"
```

The config file is looked up as `.nxrb.toml` in the working directory, so run `nxrb` from your NixOS config repo.

## Usage

```
nxrb [OPTIONS]
```

### Options

| Flag | Description |
|---|---|
| *(default)* | Build and switch to the new configuration |
| `--boot` | Build and set as the default boot entry without switching |
| `--test` | Build and activate without adding to the bootloader |
| `-u`, `--update` | Run `nix flake update` before rebuilding |
| `-n`, `--notify` | Send build result to your ntfy channel |
| `-m`, `--message <MSG>` | Custom message to include in the git commit |

### Examples

```bash
# Standard rebuild and switch
nxrb

# Update flake inputs, rebuild, and notify via ntfy
nxrb --update --notify

# Rebuild for next boot with a commit message
nxrb --boot --message "switch to zen kernel"

# Test config without touching the bootloader
nxrb --test
```

## Notifications

### D-Bus (desktop)

D-Bus notifications are declared but currently work-in-progress. The infrastructure (`notify-send` via the session bus) is in place and will be enabled in a future release.

### ntfy

When `--notify` is passed, `nxrb` posts to your configured ntfy channel with:
- A title indicating success (`✅ NixOS build completed successfully`) or failure (`❌ Failed to build NixOS`)
- The git branch name
- The error message (on failure)
- The elapsed build time

This is useful for monitoring a long rebuild on another device.

## Git workflow

When `commit_on_success = true`, after a successful rebuild `nxrb` will:

1. Switch to (or create) the configured branch with `git switch -C <branch>`
2. Stage all changes with `git add -A`
3. Commit with a message in the format `[switch] optional custom message`

When `push_on_success = true`, it then pushes to `origin/<branch>` with `--set-upstream`.

The branch field is designed to be per-device — you can maintain separate branches for each machine in your NixOS config repo.

## License

See [LICENSE](LICENSE).
