#[cfg(feature = "System+Reflection+PInfo")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PInfo {
    Attributes = 1i32,
    DeclaringType = 16i32,
    GetMethod = 2i32,
    Name = 32i32,
    ReflectedType = 8i32,
    SetMethod = 4i32,
}
#[cfg(feature = "System+Reflection+PInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::PInfo => "System.Reflection"
    ."PInfo"
);
