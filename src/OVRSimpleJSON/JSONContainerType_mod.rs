#[cfg(feature = "OVRSimpleJSON+JSONContainerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSONContainerType {
    Array = 0i32,
    Object = 1i32,
}
#[cfg(feature = "OVRSimpleJSON+JSONContainerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSONContainerType =>
    "OVRSimpleJSON"."JSONContainerType"
);
