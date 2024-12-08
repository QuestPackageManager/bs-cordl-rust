#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformComponent")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_TransformComponent {
    HAPI_TRANSFORM_QW = 9i32,
    HAPI_TRANSFORM_QX = 6i32,
    HAPI_TRANSFORM_QY = 7i32,
    HAPI_TRANSFORM_QZ = 8i32,
    HAPI_TRANSFORM_RX = 3i32,
    HAPI_TRANSFORM_RY = 4i32,
    HAPI_TRANSFORM_RZ = 5i32,
    HAPI_TRANSFORM_SX = 10i32,
    HAPI_TRANSFORM_SY = 11i32,
    HAPI_TRANSFORM_SZ = 12i32,
    HAPI_TRANSFORM_TX = 0i32,
    HAPI_TRANSFORM_TY = 1i32,
    HAPI_TRANSFORM_TZ = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_TransformComponent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_TransformComponent =>
    "HoudiniEngineUnity"."HAPI_TransformComponent"
);
