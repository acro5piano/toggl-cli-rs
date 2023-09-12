# toggl-cli-rs

CLI tool to control toggl via the api, written in Rust.

# Install

```bash
sudo curl -L https://github.com/acro5piano/toggl-cli-rs/releases/latest/download/toggl-cli-rs -o /usr/local/bin/toggl
sudo chmod +x /usr/local/bin/toggl
```

# Setup

```bash
toggl init --token <toggle_api_token>
```

# Usage

```bash
$ toggl
toggl-cli-rs 0.0.3
Toggl cli for geek

USAGE:
    toggl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    init
    list-projects
    list-timers
    start-timer
    stop-timer
    view-timer
```

### List Projects

```
toggl list-projects

# 012345678 | hobby
# 012345679 | my-project
```

### Start Timer

```
toggl start-timer --project-name my-project --description 'Daily Standup'
```

Or in short,

```
toggl start-timer -n my-project -d 'Daily Standup'
```
