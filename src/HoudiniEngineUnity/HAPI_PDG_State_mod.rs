#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_PDG_State {
    HAPI_PDG_STATE_COOKING = 1i32,
    HAPI_PDG_STATE_MAX = 2i32,
    HAPI_PDG_STATE_MAX_READY_STATE = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PDG_State =>
    "HoudiniEngineUnity"."HAPI_PDG_State"
);
