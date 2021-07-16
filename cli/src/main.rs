use std::ffi::OsStr;
use std::collections::HashMap;
use std::rc::Rc;
use std::io;
use libloading::Library;
use chimp_core::chimp_code::SCM;

pub struct SCMProxy {
    scm: Box<dyn SCM>,
    _lib: Rc<Library>
}

impl SCM for SCMProxy {
    fn clone(&self) -> String {
        self.scm.clone()
    }
}

#[derive(Default)]
pub struct ExternalSCM {
    scms: HashMap<String, SCMProxy>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalSCM {
    pub fn new() -> ExternalSCM {
        ExternalSCM::default()
    }

    pub fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
