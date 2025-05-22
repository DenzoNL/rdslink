use aws_config::profile;
use tokio; // For async main

#[tokio::main]
async fn main() {
    let profiles = get_aws_profiles().await;
    println!("AWS Profiles:");
    for profile in profiles {
        println!("{}", profile);
    }
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
