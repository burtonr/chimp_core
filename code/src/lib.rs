use chimp_core::chimp_code::SCM;
use chimp_core::CodePluginRegistrar;

chimp_core::export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn CodePluginRegistrar) {
    registrar.register_scm("github", Box::new(GitHub { owner: String::from("burtonr")}));
}

pub struct GitHub {
    pub owner: String
}

impl SCM for GitHub {
    fn clone(&self) -> std::string::String { 
        format!("Cloning GitHub repo for {}", self.owner)
    }
}
