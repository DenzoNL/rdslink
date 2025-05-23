mod ec2;
mod rds;
mod aws;
mod models;
mod port_forwarding;

use crate::port_forwarding::start_port_forwarding_session;
use crate::ec2::get_running_ec2_instances;
use inquire::Select;
use crate::rds::get_rds_instances;
use crate::aws::get_aws_config;
use crate::aws::get_aws_profiles;

#[tokio::main]
async fn main() {
    let profile_name = Select::new("Select AWS profile:", get_aws_profiles().await)
        .prompt()
        .unwrap();

    let config = get_aws_config(&profile_name).await;

    let ec2_instance = Select::new("Select EC2 instance:", get_running_ec2_instances(&config).await).prompt().unwrap();
    let rds_instance = Select::new("Select RDS instance:", get_rds_instances(&config).await).prompt().unwrap();

    start_port_forwarding_session(&profile_name, &ec2_instance.instance_id, &rds_instance.endpoint, "3306", "1053");
}

