#[cfg(feature = "HoudiniEngineUnity+HAPI_PackedPrimInstancingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_PackedPrimInstancingMode {
    HAPI_PACKEDPRIM_INSTANCING_MODE_DISABLED = 0i32,
    HAPI_PACKEDPRIM_INSTANCING_MODE_FLAT = 2i32,
    HAPI_PACKEDPRIM_INSTANCING_MODE_HIERARCHY = 1i32,
    HAPI_PACKEDPRIM_INSTANCING_MODE_INVALID = -1i32,
    HAPI_PACKEDPRIM_INSTANCING_MODE_MAX = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PackedPrimInstancingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HAPI_PackedPrimInstancingMode => "HoudiniEngineUnity"
    ."HAPI_PackedPrimInstancingMode"
);
