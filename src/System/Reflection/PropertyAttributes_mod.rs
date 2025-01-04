#[cfg(feature = "System+Reflection+PropertyAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyAttributes {
    #[default]
    HasDefault = 4096i32,
    None = 0i32,
    RTSpecialName = 1024i32,
    Reserved2 = 8192i32,
    Reserved3 = 16384i32,
    Reserved4 = 32768i32,
    ReservedMask = 62464i32,
    SpecialName = 512i32,
}
#[cfg(feature = "System+Reflection+PropertyAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::PropertyAttributes =>
    "System.Reflection"."PropertyAttributes"
);
