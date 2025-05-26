use crate::args::Args;
use crate::models::EC2Instance;
use aws_config::SdkConfig;

pub async fn get_running_ec2_instances(
    config: &SdkConfig,
    args: &Args,
) -> Result<Vec<EC2Instance>, aws_sdk_ec2::Error> {
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
        .await?;

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

    if args.ec2_filter.is_empty() {
        return Ok(instances);
    }

    Ok(instances
        .into_iter()
        .filter(|instance| {
            instance.name.as_ref().is_some_and(|name| {
                name.to_ascii_lowercase()
                    .contains(&args.ec2_filter.to_ascii_lowercase())
            })
        })
        .collect())
}
