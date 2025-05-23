mod args;
mod aws;
mod ec2;
mod models;
mod port_forwarding;
mod rds;

use crate::args::Args;
use crate::aws::get_aws_config;
use crate::aws::get_aws_profiles;
use crate::ec2::get_running_ec2_instances;
use crate::port_forwarding::start_port_forwarding_session;
use crate::rds::get_rds_instances;
use clap::Parser;
use inquire::Select;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let profile_name = Select::new("Select AWS profile:", get_aws_profiles().await)
        .prompt()
        .unwrap();

    let config = get_aws_config(&profile_name).await;

    let ec2_instance = Select::new(
        "Select EC2 instance:",
        get_running_ec2_instances(&config, &args).await,
    )
    .prompt()
    .unwrap();
    let rds_instance = Select::new("Select RDS instance:", get_rds_instances(&config).await)
        .prompt()
        .unwrap();

    start_port_forwarding_session(
        &profile_name,
        &ec2_instance.instance_id,
        &rds_instance.endpoint,
        &args.remote_port,
        &args.local_port,
    );
}
