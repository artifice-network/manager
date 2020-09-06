use std::collections::HashMap;
use std::error::Error;
use strong_xml::{XmlRead, XmlWrite};
use virt::connect::Connect;
use virt::domain::Domain;

pub struct QEMU {
    connection: Connect,
    active_domains: HashMap<String, Domain>,
}
impl QEMU {
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let connection = Connect::open("qemu:///session")?;
        let active_domains = HashMap::new();
        Ok(Self {
            connection,
            active_domains,
        })
    }
    pub fn load_by_name(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let domain = Domain::lookup_by_name(&self.connection, name)?;
        self.active_domains.insert(name.to_string(), domain);
        Ok(())
    }
    pub fn load_all_domains(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
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
    pub fn create_domain(&mut self) {}
}
impl Drop for QEMU {
    fn drop(&mut self) {
        for (_name, domain) in self.active_domains.iter_mut() {
            domain.free().unwrap();
        }
        self.connection.close().unwrap();
    }
}

#[test]
fn connect() {
    //let mut qemu = QEMU::connect().unwrap();
    //let domains = qemu.load_all_domains().unwrap();
    let xml_string = "<domain type=\"kvm\" id=\"0\"><name>Test Guest</name><uuid>lsakjdf-aseragsa-eafafd-afada</uuid><os firmware=\"efi\"><type>hvm</type><boot dev=\"cdrom\"/><boot dev=\"hd\"/></os></domain>";
    let domains = DomainConfig::from_str(xml_string).unwrap();
    println!("parsed: {:?}", domains);
    let res_string = domains.to_string().unwrap();
    println!("as string: {}", res_string);
    assert_eq!(xml_string, res_string);
    
    let os_string = "<os firmware=\"efi\"><type>hvm</type><boot dev=\"cdrom\"/><boot dev=\"hd\"/></os>";
    let os = OpSystem::from_str(os_string).unwrap();
    let os_res_str = os.to_string().unwrap();
    println!("os_res_str: {}", os_res_str);
    assert_eq!(os_string, os_res_str);
    let name = name::from_str("<name>myGuest</name>").unwrap();
    panic!();
}
#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone)]
#[xml(tag = "boot")]
pub struct boot {
    #[xml(attr = "dev")]
    dev: String,
}

macro_rules! construct_element {
    ($element:ident, $tag_name:expr, [$($attribute:ident => $attrib_type:ty => $attrib_name:expr),*], [$($child:ident => $child_type:ty => $child_name:expr),*]) => {
        #[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone)]
        #[xml(tag = $tag_name)]
        pub struct $element {
            $(
                #[xml(attr = $attrib_name)]
                pub $attribute: $attrib_type,
                #[xml(child = $child_name)]
                pub $child: $child_type,
            )*
        }
    };
    ($element:ident, $tag_name:expr, $include_type:ident, $($attribute:ident => $attrib_type:ty => $attrib_name:expr),*) => {
        #[derive(XmlRead, XmlWrite, PartialEq, Eq, Debug, Clone)]
        #[xml(tag = $tag_name)]
        pub struct $element {
            $(
                #[xml($include_type = $attrib_name)]
                pub $attribute: $attrib_type,
            )*
            #[xml(text)]
            text: String,
        }
    };
    ($($name:ident => $tag_name:expr),*) => {
        $(
            #[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone)]
            #[xml(tag = $tag_name)]
            pub struct $name {
                #[xml(text)]
                $name: String,
            }
            impl $name {
                pub fn new(name: String) -> Self{
                    Self {$name: name}
                }
            }
        )*
    };
}
construct_element!(title => "title",
    description => "description",
    genid => "genid",
    uuid => "uuid",
    name => "name"
);
construct_element! (
    r#type, "type", attr, arch => Option<String> => "arch", machine => Option<String> => "machine"
);
construct_element! (
    disk, "disk", attr, r#type => Option<String> => "type", device => Option<String> => "device"
);
construct_element! (
    OpSystem, "os", [
        firmware => Option<String> => "firmware",
        fill1 => Option<bool> => "fill1"
    ], [
        r#type => r#type => "type",
        boot => Vec<boot> => "boot"
    ]
);
construct_element! (
    DomainConfig, "domain", [
        r#type => String => "type", 
        id => Option<u32> => "id",
        fill1 => Option<bool> => "fill1",
        fill2 => Option<bool> => "fill2",
        fill3 => Option<bool> => "fill3",
        fill4 => Option<bool> => "fill4"
    ], [
        name => Option<name> => "name",
        uuid => uuid => "uuid",
        genid => Option<genid> => "genid",
        title => Option<title> => "title",
        description => Option<description> => "description",
        os => OpSystem => "os"
    ]
);
