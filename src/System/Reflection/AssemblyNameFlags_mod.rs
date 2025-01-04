#[cfg(feature = "System+Reflection+AssemblyNameFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssemblyNameFlags {
    #[default]
    EnableJITcompileOptimizer = 16384i32,
    EnableJITcompileTracking = 32768i32,
    None = 0i32,
    PublicKey = 1i32,
    Retargetable = 256i32,
}
#[cfg(feature = "System+Reflection+AssemblyNameFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyNameFlags =>
    "System.Reflection"."AssemblyNameFlags"
);
