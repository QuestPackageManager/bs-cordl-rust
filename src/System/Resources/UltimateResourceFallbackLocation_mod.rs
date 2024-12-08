#[cfg(feature = "System+Resources+UltimateResourceFallbackLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UltimateResourceFallbackLocation {
    MainAssembly = 0i32,
    Satellite = 1i32,
}
#[cfg(feature = "System+Resources+UltimateResourceFallbackLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Resources::UltimateResourceFallbackLocation => "System.Resources"
    ."UltimateResourceFallbackLocation"
);
