use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Local port to use for port forwarding
    #[arg(long, default_value = "1053")]
    pub local_port: String,

    /// Remote port to use for port forwarding
    #[arg(long, default_value = "3306")]
    pub remote_port: String,

    /// Apply filter to the EC2 instance names
    #[arg(long, default_value = "bastion")]
    pub ec2_filter: String,
}
