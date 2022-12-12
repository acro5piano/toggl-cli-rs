# toggl-cli-rs

CLI tool to control toggl via the api, written in Rust.

# Install

```bash
sudo curl -L https://github.com/acro5piano/toggl-cli-rs/releases/download/v0.0.2/toggl-cli-rs -o /usr/local/bin/toggl
sudo chmod +x /usr/local/bin/toggl
```

# Setup

```bash
toggl init --token <toggle_api_token>
```

# Usage

```bash
# List Projects
toggl list-projects

# Start Timer
toggl start-timer --pid <project_id> --description <description>

# Stop Timer
toggl stop-timer
```
