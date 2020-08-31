/*!
# Purpose

this crate is intended to be used as a pure rust cross platform virtual machine,
for not only the purposes of security, but also extensibility
!*/
#![feature(vec_into_raw_parts)]
#[macro_use]extern crate err_derive;
#[macro_use]extern crate serde_derive;
pub mod error;
pub mod opcode;

use error::FerrisError;

/*pub struct HeteroMorphicList {
    length: usize,
    size: usize,
    objects: Vec<usize>,
    base: *mut u8,
}
impl HeteroMorphicList {
    pub fn new(size: usize) -> Self{
        let mut vec: Vec<u8> = Vec::with_capacity(size);
        let ptr = unsafe {
            vec.set_len(size);
            let mut ptr = vec.into_raw_parts().0;
            ptr
        };
        Self {
            length: 0,
            size,
            objects: Vec::new(),
            base: ptr,
        }
    }
    /// types such as vec cannot be inserted using this method, since std::mem::size_of::<VEc<T>>
    /// does not account for size_of::<T> * vec.len()
    pub fn push<V>(&mut self, value: V) -> Result<(), FerrisError>{
        let obj_size = std::mem::size_of::<V>();
        if obj_size + self.length > self.size {
            return Err(FerrisError::StackOverflow);
        }
        self.objects.push(obj_size);
        unsafe { std::ptr::copy(&value as *const V as *const u8, self.base.offset(self.length as isize) as *mut u8, obj_size); }
        self.length += obj_size;
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    #[cfg(test)]
    pub (crate) fn objects(&self) -> &Vec<usize>{
        &self.objects
    }
    pub fn pop<T: Clone>(&mut self) -> Result<T, FerrisError>{
        let last_len = match self.objects.get(self.objects.len() -1) {
            Some(len) => {println!("got len: {}", len); *len},
            None => return Err(FerrisError::StackUnderflow),
        };
        let obj_len = std::mem::size_of::<T>();
        println!("about to check obj len, {}, {}", obj_len, last_len);
        if last_len != obj_len {
            return Err(FerrisError::SegFault);
        }
        let mut obj = Vec::with_capacity(last_len);
        let retval = unsafe {
            obj.set_len(last_len);
            let (ptr, _cap, _len) = obj.into_raw_parts();
            std::ptr::copy(self.base.offset(self.length as isize), ptr, last_len);
            (&*(ptr as *const T)).clone()
        };
        self.objects.pop();
        self.length -= last_len;
        Ok(retval)
    }
    pub fn extend_from_slice<T>(slice: &[T]) -> Result<(), FerrisError>{

    }
    pub fn insert<T: Clone>(index: usize) -> Result<(), FerrisError>{

    }
}
impl Drop for HeteroMorphicList {
    fn drop(&mut self) {
        let mut vector = unsafe { Vec::from_raw_parts(self.base, self.length, self.size) };
    }
}
pub struct Thread{}
pub struct Machine {
    threads: Vec<Thread>,
    memory: HeteroMorphicList,
}
#[test]
fn morphic_data () {
    let x: u64 = 0u64;
    let y: u32 = 0u32;
    let mut list = HeteroMorphicList::new(65535);
    let mut tvec: Vec<u8> = Vec::with_capacity(45);
    unsafe {tvec.set_len(45);}
    let error = FerrisError::StackOverflow;
    
    //list.push(tvec.clone());
    list.push(error.clone());
    list.push(x).unwrap();
    list.push(y).unwrap();
    
    println!("objects: {:?}", list.objects());
    let newy: u32 = list.pop().unwrap();
    assert_eq!(newy, y);
    let newx: u64 = list.pop().unwrap();
    assert_eq!(newx, x);
    let newerr: FerrisError = list.pop().unwrap();
    assert_eq!(newerr, error);
}*/