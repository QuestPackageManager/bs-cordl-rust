#[cfg(feature = "System+Configuration+Assemblies+AssemblyVersionCompatibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssemblyVersionCompatibility {
    #[default]
    SameDomain = 3i32,
    SameMachine = 1i32,
    SameProcess = 2i32,
}
#[cfg(feature = "System+Configuration+Assemblies+AssemblyVersionCompatibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Configuration::Assemblies::AssemblyVersionCompatibility =>
    "System.Configuration.Assemblies"."AssemblyVersionCompatibility"
);
