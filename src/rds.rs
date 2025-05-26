use crate::models::RDSInstance;
use aws_config::SdkConfig;
use aws_sdk_rds::{Client, Error};

pub async fn get_rds_instances(
    config: &SdkConfig,
) -> Result<Vec<RDSInstance>, Error> {
    let client = Client::new(config);

    let output = client.describe_db_instances().send().await?;

    let mut instances = vec![];

    for db_instance in output.db_instances() {
        let name = db_instance
            .db_instance_identifier()
            .unwrap_or_default()
            .to_string();
        let endpoint = db_instance
            .endpoint()
            .unwrap()
            .address()
            .unwrap_or("-")
            .to_string();
        instances.push(RDSInstance { name, endpoint });
    }

    Ok(instances)
}
