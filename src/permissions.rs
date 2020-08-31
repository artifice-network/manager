use ipnetwork::IpNetwork;
use std::fmt;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Resource {
    Read(PathBuf),
    Write(PathBuf),
    Execute(PathBuf),
    ReadWrite(PathBuf),
    WriteExecute(PathBuf),
    ReadExecute(PathBuf),
    ReadWriteExecute(PathBuf),
    Network(IpNetwork),
}
impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Read(path) => format!("r-{}", path.display()),
            Self::Write(path) => format!("w-{}", path.display()),
            Self::Execute(path) => format!("x-{}", path.display()),
            Self::ReadWrite(path) => format!("rw-{}", path.display()),
            Self::WriteExecute(path) => format!("wx-{}", path.display()),
            Self::ReadExecute(path) => format!("rx-{}", path.display()),
            Self::ReadWriteExecute(path) => format!("rwx-{}", path.display()),
            Self::Network(n) => format!("n-{}", n),
        };
        write!(f, "{}", value)
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ResourceRequest {
    resource: Resource,
    reason: Option<String>,
}
impl ResourceRequest {
    pub fn new(resource: Resource, reason: Option<String>) -> Self {
        Self { resource, reason }
    }
    pub fn resource(&self) -> &Resource {
        &self.resource
    }
    pub fn reason(&self) -> &Option<String> {
        &self.reason
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub enum RequestResult {
    Granted,
    Denied,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Permission {
    resource: Resource,
    // peer hash, application key
    granted: Vec<(String, String)>,
}
