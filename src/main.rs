use aws_config::profile;
use inquire::Select;
use tokio; // For async main

#[tokio::main]
async fn main() {
    let profile = Select::new("Select AWS profile:", get_aws_profiles().await)
        .prompt()
        .unwrap();
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
