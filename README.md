# rdslink

**rdslink** is a Rust CLI tool that simplifies connecting to AWS RDS instances by automating the setup of a secure
tunnel through AWS SSM and EC2 bastion hosts.

## How it works

**rdslink** automates the process of connecting by prompting you through the following steps:

1. **Select AWS profile**: Choose from your configured AWS profiles.
2. **List EC2 bastions**: Running EC2 instances are filtered and shown for selection.
3. **List RDS instances**: Available RDS databases are listed for you to pick.
4. **Establish tunnel**: A secure port-forwarding session is set up via SSM through the selected EC2 instance to your
   RDS
   database.

Once the tunnel is established, simply connect to the local port (default: 1053) specified in the options
and enjoy hassle-free tunnelling!

## Prerequisites

- Rust (for building)
- AWS credentials configured (via `~/.aws/config` or environment)
- SSM permissions for the target EC2 instance

## Installation (macOS)

Install via Homebrew:

```sh
$ brew tap DenzoNL/rdslink
$ brew install rdslink
```

## Installation (Windows)

For Windows, you can download the latest release from
the [GitHub releases page](https://github.com/DenzoNL/rdslink/releases)

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
- [ ] Add packaging for Windows using winget
- [ ] Add tests
- [ ] Block pushing to main branch without a PR
- [ ] Add Github Actions for CI/CD, e.g. linting, static analysis, testing, building

## License

MIT
