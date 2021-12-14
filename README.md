# toggl-cli-rs

CLI tool to control toggl via the api, written in Rust.

# Install

```bash
sudo curl -L https://github.com/acro5piano/toggl-cli-rs/releases/download/v0.0.1/toggl-cli-rs -o /usr/local/bin/toggl-cli
sudo chmod +x /usr/local/bin/toggl-cli
```

# Setup

```bash
toggl-cli init --token <toggle_api_token>
```

# Usage

```bash
# List Projects
toggl-cli list-projects

# Start Timer
toggl-cli start-timer --pid <project_id> --description <description>

# Stop Timer
toggl-cli stop-timer
```
