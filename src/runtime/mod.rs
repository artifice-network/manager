#![allow(non_camel_case_types)]
#[cfg(feature = "kvm")]
pub mod kvm;

use crate::{EnvData, EnvType, ExecEnv, RemoteEnv};
use networking::NetworkError;
use std::convert::TryInto;

pub struct VirtualEnv {
    key: [u8; 16],
    code: Vec<u8>,
    env: RemoteEnv,
}
impl VirtualEnv {
    pub fn empty(env_type: EnvType) -> Result<Self, NetworkError> {
        Ok(Self {
            key: [0; 16],
            code: Vec::new(),
            env: RemoteEnv::init(env_type)?,
        })
    }
    pub fn load(mut self, code: &[u8], key: &[u8]) -> Result<Self, NetworkError> {
        self.key = key.try_into()?;
        self.code.extend_from_slice(code);
        Ok(self)
    }
}

impl EnvData for VirtualEnv {
    fn trusted(&self) -> bool {
        self.env.trusted()
    }
    fn os_name(&self) -> &str {
        self.env.os_name()
    }
    fn arch_name(&self) -> &str {
        self.env.arch_name()
    }
    fn total_mem(&self) -> u64 {
        self.env.total_mem()
    }
    fn cpu_count(&self) -> u16 {
        self.env.cpu_count()
    }
    fn cpu_speed(&self) -> u16 {
        self.env.cpu_speed()
    }
}
impl ExecEnv for VirtualEnv {
    type Error = NetworkError;
}
pub struct NativeEnv {
    env: RemoteEnv,
}
impl NativeEnv {}
impl ExecEnv for NativeEnv {
    type Error = NetworkError;
}
impl EnvData for NativeEnv {
    fn trusted(&self) -> bool {
        self.env.trusted()
    }
    fn os_name(&self) -> &str {
        self.env.os_name()
    }
    fn arch_name(&self) -> &str {
        self.env.arch_name()
    }
    fn total_mem(&self) -> u64 {
        self.env.total_mem()
    }
    fn cpu_count(&self) -> u16 {
        self.env.cpu_count()
    }
    fn cpu_speed(&self) -> u16 {
        self.env.cpu_speed()
    }
}
