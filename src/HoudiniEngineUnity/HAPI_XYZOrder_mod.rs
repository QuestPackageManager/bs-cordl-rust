#[cfg(feature = "HoudiniEngineUnity+HAPI_XYZOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_XYZOrder {
    HAPI_XYZ = 0i32,
    HAPI_XZY = 1i32,
    HAPI_YXZ = 2i32,
    HAPI_YZX = 3i32,
    HAPI_ZXY = 4i32,
    HAPI_ZYX = 5i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_XYZOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_XYZOrder =>
    "HoudiniEngineUnity"."HAPI_XYZOrder"
);
