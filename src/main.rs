use aws_config::{profile, BehaviorVersion, SdkConfig};
use aws_sdk_ec2::operation::describe_instances::DescribeInstancesOutput;
use inquire::Select;

#[tokio::main]
async fn main() {
    let profile_name = Select::new("Select AWS profile:", get_aws_profiles().await)
        .prompt()
        .unwrap();

    let config = get_aws_config(profile_name).await;
    let instance = Select::new("Select EC2 instance:", get_running_ec2_instances(&config).await).prompt().unwrap();
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

async fn get_aws_config(profile_name: String) -> SdkConfig {
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