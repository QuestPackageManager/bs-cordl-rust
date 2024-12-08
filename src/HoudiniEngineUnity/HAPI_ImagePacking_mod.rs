#[cfg(feature = "HoudiniEngineUnity+HAPI_ImagePacking")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_ImagePacking {
    HAPI_IMAGE_PACKING_ABGR = 5i32,
    HAPI_IMAGE_PACKING_BGR = 3i32,
    HAPI_IMAGE_PACKING_DEFAULT3 = 2i32,
    HAPI_IMAGE_PACKING_DEFAULT4 = 4i32,
    HAPI_IMAGE_PACKING_DUAL = 1i32,
    HAPI_IMAGE_PACKING_MAX = 6i32,
    HAPI_IMAGE_PACKING_SINGLE = 0i32,
    HAPI_IMAGE_PACKING_UNKNOWN = -1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ImagePacking")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ImagePacking =>
    "HoudiniEngineUnity"."HAPI_ImagePacking"
);
