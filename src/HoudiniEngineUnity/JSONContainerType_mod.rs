#[cfg(feature = "HoudiniEngineUnity+JSONContainerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSONContainerType {
    Array = 0i32,
    Object = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+JSONContainerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONContainerType =>
    "HoudiniEngineUnity"."JSONContainerType"
);
