use virt::connect::Connect;
use virt::domain::Domain;
use std::collections::HashMap;
use std::error::Error;

pub struct QEMU {
    connection: Connect,
    active_domains: HashMap<String, Domain>,
}
impl QEMU {
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let connection = Connect::open("qemu:///session")?;
        let active_domains = HashMap::new();
        Ok(Self {connection, active_domains})
    }
    pub fn load_by_name(&mut self, name: &str) -> Result<(), Box<dyn Error>>{
        let domain = Domain::lookup_by_name(&self.connection, name)?;
        self.active_domains.insert(name.to_string(), domain);
        Ok(())
    }
    pub fn load_all_domains(&mut self) -> Result<Vec<String>, Box<dyn Error>>{
        let domain_list = self.connection.list_domains()?;
        let mut names = Vec::new();
        for id in domain_list.into_iter() {
            let domain = Domain::lookup_by_id(&self.connection, id)?;
            let name = domain.get_name()?;
            self.active_domains.insert(name.clone(), domain);
            names.push(name);
        }
        Ok(names)
    }
    pub fn mut_domain(&mut self, key: &str) -> Option<&mut Domain> {
        self.active_domains.get_mut(key)
    }
    pub fn create_domain(&mut self){

    }
}
impl Drop for QEMU {
    fn drop(&mut self) {
        for (_name, domain) in self.active_domains.iter_mut(){
            domain.free().unwrap();
        }
        self.connection.close().unwrap();
    }
}

#[test]
fn connect() {
    let mut qemu = QEMU::connect().unwrap();
    let domains = qemu.load_all_domains().unwrap();
    println!("{:?}", domains);
    panic!();
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainConfig {
    name: String,
    vcpu: u16,
    memory: u64,
}