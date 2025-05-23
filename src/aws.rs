use aws_config::{profile, BehaviorVersion, SdkConfig};

pub async fn get_aws_profiles() -> Vec<String> {
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

pub async fn get_aws_config(profile_name: &str) -> SdkConfig {
    aws_config::defaults(BehaviorVersion::v2025_01_17())
        .profile_name(profile_name)
        .load()
        .await
}
