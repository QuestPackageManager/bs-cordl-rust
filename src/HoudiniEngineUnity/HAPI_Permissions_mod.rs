#[cfg(feature = "HoudiniEngineUnity+HAPI_Permissions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_Permissions {
    #[default]
    HAPI_PERMISSIONS_MAX = 4i32,
    HAPI_PERMISSIONS_NON_APPLICABLE = 0i32,
    HAPI_PERMISSIONS_READ_ONLY = 2i32,
    HAPI_PERMISSIONS_READ_WRITE = 1i32,
    HAPI_PERMISSIONS_WRITE_ONLY = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Permissions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_Permissions =>
    "HoudiniEngineUnity"."HAPI_Permissions"
);
