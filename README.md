# toggl-cli-rs

CLI tool to control toggl via the api, written in Rust.

# Install

```bash
curl -L https://github.com/acro5piano/toggl-cli-rs/releases/latest/download/toggl-cli-rs -o ~/.local/bin/toggl
chmod +x ~/.local/bin/toggl

# Ensure you include the path
export PATH=$PATH:~/.local/bin/toggl
```

# Setup

```bash
toggl init --token <toggle_api_token>
```

# Usage

```bash
$ toggl
toggl-cli-rs 0.1.3
Toggl cli for geek

USAGE:
    toggl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    export-pdf
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

### Export PDF

Export time entries as a PDF report:

```
toggl export-pdf --start-date 2025-01-01 --end-date 2025-01-31 --output report.pdf
```

Or in short,

```
toggl export-pdf -s 2025-01-01 -e 2025-01-31 -o report.pdf
```

Filter by project name:

```
toggl export-pdf -s 2025-01-01 -e 2025-01-31 -n my-project -o report.pdf
```
