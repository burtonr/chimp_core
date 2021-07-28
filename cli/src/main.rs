use chimp_core::CodePluginDeclaration;
use chimp_core::chimp_code::SCM;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::rc::Rc;
use std::io;
use libloading::Library;

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

    // pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> Result<()> {
    pub unsafe fn load<P: AsRef<OsStr>>(
            &mut self,
            library_path: P,
        ) -> io::Result<()> {
        let library = Library::new(library_path);
        let library = match library {
            Ok(library) => library,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to load library",
                ))
            }
        };

        let library = Rc::new(library);
        let symbol =
            library.get::<*mut CodePluginDeclaration>(b"plugin_declaration\0");

        let symbol = match symbol {
            Ok(symbol) => symbol,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to load plugin_declaration symbol",
                ))
            }
        };

        let decl = symbol.read();

        if decl.rustc_version != chimp_core::RUSTC_VERSION
            || decl.core_version != chimp_core::CORE_VERSION
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Version mismatch",
            ));
        }
        let mut registrar = SCMRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);
        // add all loaded plugins to the functions map
        self.scms.extend(registrar.scms);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);
        Ok(())
        // let library = Rc::new(Library::new(library_path)?);
        // let decl = 
        //     library
        //         .get::<*mut CodePluginDeclaration>(b"plugin_declaration\0")?
        //         .read();

        // if decl.rustc_version != chimp_core::RUSTC_VERSION
        //     || decl.core_version != chimp_core::CORE_VERSION {
        //         // return Err(io::Error::new(io::ErrorKind::Other, "Version Mismatch",));
        //         return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Version Mismatch",)));
        // }

        // let mut registrar = CodePluginRegistrar::new(Rc::clone(&library));

        // (decl.register)(&mut registrar);

        // self.scms.extend(registrar.scms);
        // self.scms.push(library);

        // Ok(())
    }
}

struct SCMRegistrar {
    scms: HashMap<String, SCMProxy>,
    lib: Rc<Library>,
}

impl SCMRegistrar {
    fn new(lib: Rc<Library>) -> SCMRegistrar {
        SCMRegistrar {
            lib,
            scms: HashMap::default(),
        }
    }
}

impl chimp_core::CodePluginRegistrar for SCMRegistrar {
    fn register_scm(&mut self, name: &str, scm: Box<dyn chimp_core::chimp_code::SCM>) {
        let proxy = SCMProxy {
            scm,
            _lib: Rc::clone(&self.lib),
        };
        self.scms.insert(name.to_string(), proxy);
    }
}

fn main() {
    println!("Hello, world!");
}
