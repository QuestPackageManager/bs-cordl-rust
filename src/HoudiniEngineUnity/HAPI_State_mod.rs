#[cfg(feature = "HoudiniEngineUnity+HAPI_State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_State {
    #[default]
    HAPI_STATE_COOKING = 4i32,
    HAPI_STATE_LOADING = 6i32,
    HAPI_STATE_MAX = 7i32,
    HAPI_STATE_MAX_READY_STATE = 2i32,
    HAPI_STATE_READY = 0i32,
    HAPI_STATE_READY_WITH_FATAL_ERRORS = 1i32,
    HAPI_STATE_STARTING_COOK = 3i32,
    HAPI_STATE_STARTING_LOAD = 5i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_State =>
    "HoudiniEngineUnity"."HAPI_State"
);
