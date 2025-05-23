#[derive(Debug)]
pub struct EC2Instance {
    pub instance_id: String,
    pub name: Option<String>,
}

impl std::fmt::Display for EC2Instance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "{} ({})",
            self.instance_id,
            self.name.as_deref().unwrap_or("-")
        )
    }
}

#[derive(Debug)]
pub struct RDSInstance {
    pub name: String,
    pub endpoint: String,
}

impl std::fmt::Display for RDSInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} ({})", self.name, self.endpoint)
    }
}
