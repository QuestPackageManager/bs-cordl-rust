#[cfg(feature = "System+Reflection+EventAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventAttributes {
    None = 0i32,
    RTSpecialName = 1024i32,
    SpecialName = 512i32,
}
#[cfg(feature = "System+Reflection+EventAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::EventAttributes =>
    "System.Reflection"."EventAttributes"
);
