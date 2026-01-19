use aws_config::profile::ProfileFileLoadError;
use aws_config::{BehaviorVersion, SdkConfig, profile};

pub async fn get_aws_profiles() -> Result<Vec<String>, ProfileFileLoadError> {
    let profile_set = profile::load(
        &Default::default(),
        &Default::default(),
        &Default::default(),
        None,
    )
    .await?;

    let mut profiles: Vec<String> = profile_set.profiles().map(ToString::to_string).collect();
    profiles.sort();
    Ok(profiles)
}

pub async fn get_aws_config(profile_name: &str) -> SdkConfig {
    aws_config::defaults(BehaviorVersion::v2026_01_12())
        .profile_name(profile_name)
        .load()
        .await
}

pub async fn validate_aws_config(config: &aws_config::SdkConfig) -> Result<(), String> {
    let client = aws_sdk_sts::Client::new(config);
    match client.get_caller_identity().send().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e}")),
    }
}
