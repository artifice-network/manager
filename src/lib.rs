#[macro_use]
extern crate serde_derive;

#[cfg(feature = "hashdatabase")]
pub mod database;

#[cfg(target_os = "Windows")]
unimplemented!();
pub mod applications;
pub mod permissions;
pub mod runtime;
use networking::{
    asyncronous::AsyncStream, syncronous::SyncStream, ArtificeConfig, ArtificePeer, LongHash,
    NetworkError, NetworkHash,
};
use std::collections::HashMap;

/// the first two environemnts that will be implemented are MeSHE using Paillier, and Trusted, or execution directly on the host
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvType {
    Inherit,
    Paillier,
    Other(String),
}
/// data on execution environments including cpu, memory, type, architecture, system, and trust
pub trait EnvData {
    /// assume untrusted because the purpose of this project is to create artificially trusted
    /// systems, rather then knowing they are trusted by their identity
    fn trusted(&self) -> bool {
        false
    }
    fn env_type(&self) -> &EnvType {
        &EnvType::Inherit
    }
    /// the following methods don't have a default implementation
    /// because any default would be based on the local system, not the remote system
    fn os_name(&self) -> &str;
    fn arch_name(&self) -> &str;
    fn total_mem(&self) -> u64;
    fn cpu_count(&self) -> u16;
    fn cpu_speed(&self) -> u16;
}
/// struct that implements EnvData, and can be used to store info on remote hosts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteEnv {
    os_name: String,
    arch_name: String,
    // 1TB > 2^32 || 1TB > u32
    total_mem: u64,
    // current ryzen max 128 core, future > 256 == 2^8
    cpu_count: u16,
    // measured in MHz, petahertz > 65535 == 2^16 but petahertz not likely1
    cpu_speed: u16,
    env_type: EnvType,
    trusted: bool,
}
impl RemoteEnv {
    pub fn init(env_type: EnvType) -> Result<Self, NetworkError> {
        let rust_info = rust_info::get();
        let os_name = match rust_info.target_os {
            Some(os) => os,
            None => return Err(NetworkError::UnSet(String::from("target_os not set"))),
        };
        let arch_name = match rust_info.target_arch {
            Some(arch) => arch,
            None => return Err(NetworkError::UnSet(String::from("target_arch not set"))),
        };
        let total_mem = match sys_info::mem_info() {
            Ok(mem) => mem.total,
            Err(e) => return Err(NetworkError::UnSet(format!("{}", e))),
        };
        let cpu_count = match sys_info::cpu_num() {
            Ok(num) => num as u16,
            Err(e) => return Err(NetworkError::UnSet(format!("{}", e))),
        };
        let cpu_speed = match sys_info::cpu_speed() {
            Ok(speed) => speed as u16,
            Err(e) => return Err(NetworkError::UnSet(format!("{}", e))),
        };
        Ok(Self {
            os_name,
            arch_name,
            total_mem,
            cpu_count,
            cpu_speed,
            env_type,
            trusted: false,
        })
    }
    /// use to set trusted to true, default is false
    pub fn set_trusted(mut self, trusted: bool) -> Self {
        self.trusted = trusted;
        self
    }
}
/// used to keep track of env data + extra data on localhost
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostEnv {
    core: RemoteEnv,
    config: ArtificeConfig,
    public: bool,
}
impl HostEnv {
    /// # Arguments
    ///
    /// config: Local ArtificeConfig click for more info
    /// env_type: passed into RemoteEnv when getting data local env
    /// public: should this env be a shared resource, not that this will only be allowed, once secure exec environments become possible
    pub fn init(
        config: ArtificeConfig,
        env_type: EnvType,
        public: bool,
    ) -> Result<Self, NetworkError> {
        let core = RemoteEnv::init(env_type)?;
        Ok(Self {
            core,
            config,
            public,
        })
    }
    /// # Arguments
    /// same arguments as init, but you have more control over the environment data passed in
    pub fn new(core: RemoteEnv, config: ArtificeConfig, public: bool) -> Self {
        Self {
            core,
            config,
            public,
        }
    }
    pub fn env_data(&self) -> &RemoteEnv {
        &self.core
    }
    pub fn mut_env_data(&mut self) -> &mut RemoteEnv {
        &mut self.core
    }
    pub fn config(&self) -> &ArtificeConfig {
        &self.config
    }
    pub fn mut_config(&mut self) -> &mut ArtificeConfig {
        &mut self.config
    }
    pub fn is_public(&self) -> bool {
        self.public
    }
    pub fn set_public(&mut self, public: bool) {
        self.public = public;
    }
}
impl EnvData for HostEnv {
    fn trusted(&self) -> bool {
        self.core.trusted()
    }
    fn os_name(&self) -> &str {
        self.core.os_name()
    }
    fn arch_name(&self) -> &str {
        self.core.arch_name()
    }
    fn total_mem(&self) -> u64 {
        self.core.total_mem()
    }
    fn cpu_count(&self) -> u16 {
        self.core.cpu_count()
    }
    fn cpu_speed(&self) -> u16 {
        self.core.cpu_speed()
    }
}
/// used to hold information on remote  systems, by keeping track of EnvData, and a ArtificePeer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteHost {
    peer: ArtificePeer,
    data: RemoteEnv,
}
impl RemoteHost {
    pub fn new(peer: ArtificePeer, data: RemoteEnv) -> Self {
        Self { peer, data }
    }
    pub fn peer(&self) -> &ArtificePeer {
        &self.peer
    }
    /// can be used to update data of a given peer
    pub fn mut_peer(&mut self) -> &mut ArtificePeer {
        &mut self.peer
    }
    pub fn env_data(&self) -> &RemoteEnv {
        &self.data
    }
    /// can be used to update data on a given environment
    pub fn mut_env_data(&mut self) -> &mut RemoteEnv {
        &mut self.data
    }
    /// doesn't store the stream internally because this struct should be serialize, and deserialize
    pub fn sync_connect(&self) -> Result<SyncStream, NetworkError> {
        SyncStream::connect(&self.peer)
    }
    /// doesn't store the stream internally because this struct should be serialize, and deserialize
    pub async fn async_connect(&self) -> Result<AsyncStream, NetworkError> {
        AsyncStream::connect(&self.peer).await
    }
}
impl LongHash for RemoteHost {
    fn hash(&self) -> &NetworkHash {
        self.peer.global_peer_hash()
    }
}
impl EnvData for RemoteEnv {
    fn trusted(&self) -> bool {
        self.trusted
    }
    fn env_type(&self) -> &EnvType {
        &self.env_type
    }
    fn os_name(&self) -> &str {
        &self.os_name
    }
    fn arch_name(&self) -> &str {
        &self.arch_name
    }
    fn total_mem(&self) -> u64 {
        self.total_mem
    }
    fn cpu_count(&self) -> u16 {
        self.cpu_speed
    }
    fn cpu_speed(&self) -> u16 {
        self.cpu_speed
    }
}

pub trait ExecEnv: EnvData {
    type Error: std::error::Error;
    /// check available memory count
    fn current_mem(&self) -> Result<u64, Self::Error>
    where
        Self::Error: From<sys_info::Error>,
    {
        match sys_info::mem_info() {
            Ok(mem) => Ok(mem.free),
            Err(e) => Err(e.into()),
        }
    }
    /// get data on how much cpu is being used
    fn load_avg(&self) -> Result<f64, Self::Error>
    where
        Self::Error: From<sys_info::Error>,
    {
        match sys_info::loadavg() {
            Ok(avg) => Ok(avg.fifteen),
            Err(e) => Err(e.into()),
        }
    }
}
/// this is the main struct of this library, and is used to run code on remote systems
pub struct Distributor<E: ExecEnv> {
    database: HashMap<NetworkHash, RemoteHost>,
    connections: HashMap<NetworkHash, AsyncStream>,
    env: Option<E>,
}
impl<E: ExecEnv> Distributor<E> {
    pub fn empty() -> Self {
        Self {
            database: HashMap::new(),
            env: None,
            connections: HashMap::new(),
        }
    }
    /// # Arguments
    ///
    /// database: record of peers, indexed by the global peer hash
    /// env: execution environment to use on the remote system
    pub fn load(database: HashMap<NetworkHash, RemoteHost>, env: E) -> Self {
        Self {
            database,
            env: Some(env),
            connections: HashMap::new(),
        }
    }
    /// load peers
    pub fn database(mut self, database: HashMap<NetworkHash, RemoteHost>) -> Self {
        for (name, record) in database.into_iter() {
            self.database.insert(name, record);
        }
        self
    }
    /// select execution environment
    pub fn env(mut self, env: E) -> Self {
        self.env = Some(env);
        self
    }
    /// return peers, and select execution environment
    pub fn collapse(self) -> (HashMap<NetworkHash, RemoteHost>, E) {
        (self.database, self.env.unwrap())
    }
    /// attempts to connect to remote host using the global hash of a peer
    pub async fn connect(&mut self, hash: &NetworkHash) -> Result<(), NetworkError> {
        let stream = match self.database.get(hash) {
            Some(host) => host.async_connect().await?,
            None => return Err(NetworkError::UnSet(String::from("Couldn't Find Peer"))),
        };
        self.connections.insert(*hash, stream);
        Ok(())
    }
    /// connect to list of peers
    pub async fn establish(&mut self, hashes: &[NetworkHash]) -> Result<(), NetworkError> {
        for hash in hashes {
            self.connect(&hash).await?;
        }
        Ok(())
    }
    pub fn append_incoming(&mut self, stream: AsyncStream) {
        self.connections.insert(*stream.hash(), stream);
    }
}
