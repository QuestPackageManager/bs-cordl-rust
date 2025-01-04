#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxations")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompilationRelaxations {
    #[default]
    NoStringInterning = 8i32,
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxations")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::CompilationRelaxations =>
    "System.Runtime.CompilerServices"."CompilationRelaxations"
);
