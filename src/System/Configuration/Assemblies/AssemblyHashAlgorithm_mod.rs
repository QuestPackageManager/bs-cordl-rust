#[cfg(feature = "System+Configuration+Assemblies+AssemblyHashAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssemblyHashAlgorithm {
    MD5 = 32771i32,
    None = 0i32,
    SHA1 = 32772i32,
    SHA256 = 32780i32,
    SHA384 = 32781i32,
    SHA512 = 32782i32,
}
#[cfg(feature = "System+Configuration+Assemblies+AssemblyHashAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Configuration::Assemblies::AssemblyHashAlgorithm =>
    "System.Configuration.Assemblies"."AssemblyHashAlgorithm"
);
