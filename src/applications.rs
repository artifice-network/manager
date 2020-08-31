use crate::permissions::Resource;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppIdentity {
    name: String,
    version: String,
    api_key: String,
}
impl AppIdentity {
    pub fn new(name: String, version: String, api_key: String) -> Self {
        Self {
            name,
            version,
            api_key,
        }
    }
}
impl fmt::Display for AppIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    name: String,
    version: String,
    api_key: String,
    owner: String,
    permissions: Vec<Resource>,
    description: Option<String>,
    documentation: Option<String>,
    platform: Option<String>,
    repository: Option<String>,
}
impl Application {
    pub fn new(name: String, version: String, api_key: String, owner: String) -> Self {
        Self {
            name,
            version,
            api_key,
            owner,
            permissions: Vec::new(),
            description: None,
            documentation: None,
            platform: None,
            repository: None,
        }
    }
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn permissions(mut self, resources: &[Resource]) -> Self {
        self.permissions.extend_from_slice(resources);
        self
    }
    pub fn documentation(mut self, docs: String) -> Self {
        self.documentation = Some(docs);
        self
    }
    pub fn platform(mut self, platform: String) -> Self {
        self.platform = Some(platform);
        self
    }
    pub fn repository(mut self, repo: String) -> Self {
        self.repository = Some(repo);
        self
    }
    pub fn identity(&self) -> AppIdentity {
        AppIdentity::new(
            self.name.clone(),
            self.version.clone(),
            self.api_key.clone(),
        )
    }
}
