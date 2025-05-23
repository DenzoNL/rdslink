use std::process::Command;
use aws_config::{profile, BehaviorVersion, SdkConfig};
use inquire::Select;

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

async fn get_aws_profiles() -> Vec<String> {
    let profile_set = profile::load(
        &Default::default(),
        &Default::default(),
        &Default::default(),
        None,
    )
    .await
    .unwrap();

    profile_set.profiles().map(ToString::to_string).collect()
}

async fn get_aws_config(profile_name: &str) -> SdkConfig {
    aws_config::defaults(BehaviorVersion::v2025_01_17()).profile_name(profile_name).load().await
}

#[derive(Debug)]
struct EC2Instance {
    instance_id: String,
    name: Option<String>,
}

impl std::fmt::Display for EC2Instance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} ({})", self.instance_id, self.name.as_deref().unwrap_or("-"))
    }
}

#[derive(Debug)]
struct RDSInstance {
    name: String,
    endpoint: String
}

impl std::fmt::Display for RDSInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} ({})", self.name, self.endpoint)
    }
}

async fn get_running_ec2_instances(config: &SdkConfig) -> Vec<EC2Instance> {
    let client = aws_sdk_ec2::Client::new(config);

    let output = client
        .describe_instances()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("instance-state-name")
                .values("running")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let mut instances = vec![];

    for reservation in output.reservations() {
        for instance in reservation.instances() {
            let instance_id = instance.instance_id().unwrap_or_default().to_string();
            let name = instance
                .tags()
                .iter()
                .find(|tag| tag.key() == Some("Name"))
                .and_then(|tag| tag.value().map(|s| s.to_string()));

            instances.push(EC2Instance { instance_id, name });
        }
    }
    
    instances
}

async fn get_rds_instances(config: &SdkConfig) -> Vec<RDSInstance> {
    let client = aws_sdk_rds::Client::new(config);

    let output = client
        .describe_db_instances()
        .send()
        .await
        .unwrap();

    let mut instances = vec![];

    for db_instance in output.db_instances() {
        let name = db_instance.db_instance_identifier().unwrap_or_default().to_string();
        let endpoint = db_instance.endpoint().unwrap().address().unwrap_or("-").to_string();
        instances.push(RDSInstance { name, endpoint });
    }

    instances
}

fn start_port_forwarding_session(
    profile_name: &str,
    instance_id: &str,
    rds_host: &str,
    remote_port: &str,
    local_port: &str,
) {
    let parameters = format!(
        "{{\"portNumber\":[\"{}\"],\"localPortNumber\":[\"{}\"],\"host\":[\"{}\"]}}",
        remote_port, local_port, rds_host
    );

    let status = Command::new("aws")
        .arg("ssm")
        .arg("start-session")
        .arg("--target")
        .arg(instance_id)
        .arg("--profile")
        .arg(profile_name)
        .arg("--document-name")
        .arg("AWS-StartPortForwardingSessionToRemoteHost")
        .arg("--parameters")
        .arg(&parameters)
        .status()
        .expect("failed to execute aws cli");

    if status.success() {
        println!("Port forwarding session started.");
    } else {
        println!("Failed to start port forwarding session.");
    }
}