mod args;
mod aws;
mod ec2;
mod models;
mod port_forwarding;
mod rds;

use crate::args::Args;
use crate::aws::*;
use crate::ec2::get_running_ec2_instances;
use crate::port_forwarding::start_port_forwarding_session;
use crate::rds::get_rds_instances;
use clap::Parser;
use inquire::Select;
use std::net::TcpListener;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    ensure_port_available(&args.local_port);

    let profiles = match get_aws_profiles().await {
        Ok(profiles) => {
            if profiles.is_empty() {
                eprintln!("No AWS profiles found. Please configure your AWS CLI.");
                exit(1);
            }
            profiles
        }
        Err(e) => {
            eprintln!("Error fetching AWS profiles: {e}");
            exit(1);
        }
    };

    let profile_name = match Select::new("Select AWS profile:", profiles).prompt() {
        Ok(profile) => profile,
        Err(inquire::error::InquireError::OperationInterrupted) => {
            println!("Selection cancelled. Exiting application.");
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let config = get_aws_config(&profile_name).await;

    if let Err(e) = validate_aws_config(&config).await {
        if e.contains("dispatch failure") {
            eprintln!(
                "Could not validate AWS config: {e}\n\
            This may mean your AWS SSO session has expired or you are not logged in.\n\
            To login, please run:\n\n\
            aws sso login --profile {profile_name}"
            );
        } else {
            eprintln!(
                "Error validating AWS config: {e}.\n\nPlease check your AWS credentials and configuration."
            );
        }
        exit(1);
    }

    let ec2_instances = match get_running_ec2_instances(&config, &args).await {
        Ok(instances) => instances,
        Err(e) => {
            eprintln!("An error occurred while fetching EC2 instances: {e}");
            exit(1);
        }
    };

    if ec2_instances.is_empty() {
        eprintln!("No running EC2 instances found.");
        exit(1);
    }

    let ec2_instance = match Select::new("Select EC2 instance:", ec2_instances).prompt() {
        Ok(instance) => instance,
        Err(inquire::error::InquireError::OperationInterrupted) => {
            println!("Selection cancelled. Exiting application.");
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let rds_instances = match get_rds_instances(&config).await {
        Ok(instances) => {
            if instances.is_empty() {
                eprintln!("No RDS instances found.");
                exit(1);
            }
            instances
        }
        Err(e) => {
            eprintln!("An error occurred while fetching RDS instances: {e}");
            exit(1);
        }
    };

    let rds_instance = match Select::new("Select RDS instance:", rds_instances).prompt() {
        Ok(instance) => instance,
        Err(inquire::error::InquireError::OperationInterrupted) => {
            println!("Selection cancelled. Exiting application.");
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    start_port_forwarding_session(
        &profile_name,
        &ec2_instance.instance_id,
        &rds_instance.endpoint,
        &args.remote_port,
        &args.local_port,
    );
}

fn ensure_port_available(port: &String) {
    match TcpListener::bind(format!("127.0.0.1:{port}")) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "Port {port} is already in use.\n\nPlease close the application using this port or specify a different port using the --local-port option.",
            );
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                eprintln!(
                    "If you want to kill the process using this port, you can run:\n\n\
                    kill -9 $(lsof -t -i :{port})"
                );
            }
            exit(1)
        }
    }
}
