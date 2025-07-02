use std::process::Command;

pub fn start_port_forwarding_session(
    profile_name: &str,
    instance_id: &str,
    rds_host: &str,
    remote_port: &str,
    local_port: &str,
) {
    let parameters = format!(
        "{{\"portNumber\":[\"{remote_port}\"],\"localPortNumber\":[\"{local_port}\"],\"host\":[\"{rds_host}\"]}}"
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
