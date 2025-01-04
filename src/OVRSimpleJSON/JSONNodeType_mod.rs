#[cfg(feature = "OVRSimpleJSON+JSONNodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JSONNodeType {
    #[default]
    Array = 1i32,
    Boolean = 6i32,
    Custom = 255i32,
    None = 7i32,
    NullValue = 5i32,
    Number = 4i32,
    Object = 2i32,
    String = 3i32,
}
#[cfg(feature = "OVRSimpleJSON+JSONNodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSONNodeType => "OVRSimpleJSON"
    ."JSONNodeType"
);
