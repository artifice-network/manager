/*
// the purpose of this file was to use libvirt to manage virtual machines
// but due to the complexity of creating an xml generator suited to kvm
// I instead opted to use the qemu command line interface to start virtual machines for now
*/
use std::collections::HashMap;
use std::error::Error;
use strong_xml::{XmlRead, XmlWrite};
use virt::connect::Connect;
use virt::domain::Domain;
use crate::runtime::Unit;

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
    let acpi = acpi::default();
    println!("acpi: {}", acpi.to_string().unwrap());
    let res_acpi = acpi::from_str("<acpi/>").unwrap();
    assert_eq!(acpi, res_acpi);
    let xml_string = "
<domain type=\"kvm\" id=\"0\">
<name>Test Guest</name>
<uuid>lsakjdf-aseragsa-eafafd-afada</uuid>
<memory unit=\"MiB\">4000</memory>
<vcpu placement=\"static\">4</vcpu>
<os firmware=\"efi\">
<type>hvm</type>
<boot dev=\"cdrom\"/>
<boot dev=\"hd\"/>
</os>
<cpu mode=\"host-model\" check=\"partial\">
<model fallback=\"allow\"/>
</cpu>
</domain>";
    let domains = DomainConfig::from_str(xml_string).unwrap();
    println!("parsed: {:?}", domains);
    let res_string = domains.to_string().unwrap();
    println!("as string: {}", res_string);
    assert_eq!(xml_string.replace("\n", ""), res_string);

    let os_string =
        "<os firmware=\"efi\"><type>hvm</type><boot dev=\"cdrom\"/><boot dev=\"hd\"/></os>";
    let os = OpSystem::from_str(os_string).unwrap();
    let os_res_str = os.to_string().unwrap();
    println!("os_res_str: {}", os_res_str);
    assert_eq!(os_string, os_res_str);
    let name = name::from_str("<name>myGuest</name>").unwrap();
    panic!();
}
#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[xml(tag = "boot")]
pub struct boot {
    #[xml(attr = "dev")]
    dev: String,
}
/// this struct is designed for creating configurations so the types
/// used in it should implement either Display or XmlRead & XmlWrite, as well as Serialize, and Deserialize
macro_rules! construct_element {
    ($element:ident, $tag_name:expr, [$($attribute:ident => $attrib_type:ty => $attrib_name:expr),*], [$($child:ident => $child_type:ty => $child_name:expr),*]) => {
        #[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
        #[xml(tag = $tag_name)]
        pub struct $element {
            $(
                #[xml(attr = $attrib_name)]
                pub $attribute: $attrib_type,
            )*
            $(
                #[xml(child = $child_name)]
                pub $child: $child_type,
            )*
        }
    };
    ($element:ident, $tag_name:expr, $include_type:ident, $($attribute:ident => $attrib_type:ty => $attrib_name:expr),*) => {
        #[derive(XmlRead, XmlWrite, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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
            #[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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
    ($no_text:ty, $($name:ident => $tag_name:expr),*) => {
        $(
            #[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
            #[xml(tag = $tag_name)]
            pub struct $name {}
            impl Default for $name {
                fn default() -> Self {
                    Self{}
                }
            }
        )*
    };
    ($no_text:ty, $element:ident, $tag_name:expr, $include_type:ident, $($attribute:ident => $attrib_type:ty => $attrib_name:expr),*) => {
        #[derive(XmlRead, XmlWrite, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
        #[xml(tag = $tag_name)]
        pub struct $element {
            $(
                #[xml($include_type = $attrib_name)]
                pub $attribute: $attrib_type,
            )*
        }
    };
}

// =============================
//      Simple Elements
// =============================

construct_element!(title => "title",
    description => "description",
    genid => "genid",
    uuid => "uuid",
    name => "name",
    emulator => "emulator",
    on_poweroff => "on_powerof",
    on_reboot => "on_reboot",
    on_crash => "on_crash"
);
impl Default for on_poweroff {
    fn default() -> Self {
        Self {on_poweroff: String::from("destroy")}
    }
}

impl Default for on_reboot {
    fn default() -> Self {
        Self {on_reboot: String::from("restart")}
    }
}

impl Default for on_crash {
    fn default() -> Self {
        Self {on_crash: String::from("destroy")}
    }
}

// ==============================
//      Tag Only
// ==============================

construct_element! (bool, 
    readonly => "readonly",
    acpi => "acpi",
    apic => "apic"
);

// =========================
//      Medium Elements
// =========================

construct_element! (
    r#type, "type", attr, arch => Option<String> => "arch", machine => Option<String> => "machine"
);
construct_element! (
    vmport, "vmport", attr, state => Option<String> => "state"
);
construct_element! (
    driver, "driver", attr, name => Option<String> => "name", r#type => Option<String> => "type"
);
construct_element! (
    Target, "target", attr, dev => Option<String> => "dev", bus => Option<String> => "bus"
);
construct_element! (
    Source, "source", attr, file => Option<String> => "file"
);
construct_element! (
    memory, "memory", attr,
    unit => Unit => "unit"
);
construct_element! (
    vcpu, "vcpu", attr,
    placement => Option<String> => "placement"
);
construct_element! {
    Timer, "timer", attr,
    name => Option<String> => "name",
    tickpolicy => Option<String> => "tickpolicy",
    present => Option<String> => "present"
}

construct_element! (
    Features, "features", child,
    acpi => Option<acpi> => "acpi",
    apic => Option<apic> => "apic",
    vmport => Option<vmport> => "vmport"
);
construct_element! (
    PM, "pm", child,
    suspend_to_mem => Option<SuspendToMem> => "suspend-to-mem",
    suspend_to_disk => Option<SuspendToDisk> => "suspend-to-disk"
);
construct_element! (
    Clock, "clock", [
        offset => Option<String> => "offset"
    ], [
        timers => Vec<Timer> => "timer"
    ]
);

construct_element! (bool,
    Model, "model", attr,
    fallback => Option<String> => "fallback"
);
construct_element! (bool,
    SuspendToMem, "suspend-to-mem", attr,
    enabled => Option<String> => "enabled"
);
construct_element! (bool,
    SuspendToDisk, "suspend-to-disk", attr,
    enabled => Option<String> => "enabled"
);

construct_element! (
    CPU, "cpu", [
        mode => Option<String> => "mode",
        check => Option<String> => "check"
    ], [
        model => Option<Model> => "model"
    ]
);

// =========================
//      Address
// =========================

construct_element! (
    Address, "address", attr, 
    r#type => Option<String> => "type", 
    domain => Option<String> => "domain", 
    bus => Option<String> => "bus",
    slot => Option<String> => "slot",
    function => Option<String> => "function",
    controller => Option<u16> => "controller",
    target => Option<u16> => "target",
    unit => Option<u32> => "unit",
    port => Option<u16> => "port"
);

// ============================
//         Disk
// ============================
construct_element! (
    Disk, "disk", [
        r#type => Option<String> => "type", 
        device => Option<String> => "device"
    ], [
        driver => Option<driver> => "driver",
        source => Option<Source> => "source",
        target => Option<Target> => "target",
        address => Option<Address> => "address"
    ]
);

// ================================
//      OpSystem
// ================================

construct_element! ();
construct_element! (
    OpSystem, "os", [
        firmware => Option<String> => "firmware"
    ], [
        r#type => r#type => "type",
        boot => Vec<boot> => "boot"
    ]
);



// ==================================
//      Device
// ==================================

construct_element! (
    Controller, "controller", [
        attrib_type => Option<String> => "type",
        index => u32 => "index",
        model => Option<String> => "model"
    ], [
        address => Address => "address",
        
    ]
);

construct_element! (
    Devices, "devices", child,
    disks => Vec<Disk> => "disk",
    controllers => Vec<Controller> => "controller"
);

// ==================================
//      DomainConfig
// ==================================

construct_element! (
    DomainConfig, "domain", [
        r#type => String => "type",
        id => Option<u32> => "id"
    ], [
        name => Option<name> => "name",
        uuid => uuid => "uuid",
        genid => Option<genid> => "genid",
        title => Option<title> => "title",
        description => Option<description> => "description",
        memory => memory => "memory",
        vcpu => vcpu => "vcpu",
        os => OpSystem => "os",
        features => Option<Features> => "features",
        cpu => Option<CPU> => "cpu",
        clock => Option<Clock> => "clock",
        on_poweroff => Option<on_poweroff> => "on_poweroff",
        on_reboot => Option<on_reboot> => "on_reboot",
        on_crash => Option<on_crash> => "on_crash",
        pm => Option<PM> => "pm"
    ]
);
impl DomainConfig {
    pub fn mut_os(&mut self) -> &mut OpSystem {
        &mut self.os
    }
    pub fn os(&self) -> &OpSystem {
        &self.os
    }
}
// difference between networking and ssh (other then pnet) is ssh can have leaked public key problems
// if public key is stole (easy) then a hacker could send public key to a host that has authed
// that key, as a result they can gane enter (even if the can't view the results)