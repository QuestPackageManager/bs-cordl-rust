#[cfg(feature = "System+Reflection+ParameterAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParameterAttributes {
    #[default]
    HasDefault = 4096i32,
    HasFieldMarshal = 8192i32,
    In = 1i32,
    Lcid = 4i32,
    None = 0i32,
    Optional = 16i32,
    Out = 2i32,
    Reserved3 = 16384i32,
    Reserved4 = 32768i32,
    ReservedMask = 61440i32,
    Retval = 8i32,
}
#[cfg(feature = "System+Reflection+ParameterAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::ParameterAttributes =>
    "System.Reflection"."ParameterAttributes"
);
