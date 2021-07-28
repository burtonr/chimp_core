pub mod chimp_code;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait CodePluginRegistrar {
    fn register_scm(&mut self, name: &str, scm: Box<dyn chimp_code::SCM>);
}

#[derive(Copy, Clone)]
pub struct CodePluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn CodePluginRegistrar)
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::CodePluginDeclaration = 
            $crate::CodePluginDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                core_version: $crate::CORE_VERSION,
                register: $register,
            };
    };
}
