#[cfg(feature = "System+Reflection+CallingConventions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConventions {
    Any = 3i32,
    ExplicitThis = 64i32,
    HasThis = 32i32,
    Standard = 1i32,
    VarArgs = 2i32,
}
#[cfg(feature = "System+Reflection+CallingConventions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CallingConventions =>
    "System.Reflection"."CallingConventions"
);
