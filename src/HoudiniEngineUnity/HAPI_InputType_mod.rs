#[cfg(feature = "HoudiniEngineUnity+HAPI_InputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_InputType {
    #[default]
    HAPI_INPUT_GEOMETRY = 1i32,
    HAPI_INPUT_INVALID = -1i32,
    HAPI_INPUT_MAX = 2i32,
    HAPI_INPUT_TRANSFORM = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_InputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_InputType =>
    "HoudiniEngineUnity"."HAPI_InputType"
);
