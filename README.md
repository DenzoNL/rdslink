# rdslink

**rdslink** is a Rust CLI tool that simplifies connecting to AWS RDS instances by automating the setup of a secure
tunnel through AWS SSM and EC2 bastion hosts.

## Features

- Lists running EC2 instances with optional name filtering
- Supports AWS profile selection
- Establishes port forwarding to RDS via SSM Session Manager
- Simple command-line interface

## Prerequisites

- Rust (for building)
- AWS credentials configured (via `~/.aws/config` or environment)
- SSM permissions for the target EC2 instance

## Installation

Clone the repository and build with Cargo:

```sh
git clone <repo-url>
cd rdslink
cargo build --release
```

## Usage

```sh
rdslink [OPTIONS]
```

### Options

- `--local-port <PORT>`: Local port for forwarding (default: 1053)
- `--remote-port <PORT>`: Remote port on RDS (default: 3306)
- `--ec2-filter <FILTER>`: Filter EC2 instances by name (default: bastion)

Example:

```sh
rdslink --local-port 15432 --remote-port 5432 --ec2-filter my-bastion
```

This will list running EC2 instances whose name contains "my-bastion", prompt for AWS profile selection, and set up port
forwarding from local port 15432 to the RDS instance via the selected bastion.

## Todo list

- [ ] Refactor Copilot code to be a bit simpler and cleaner

## License

MIT
