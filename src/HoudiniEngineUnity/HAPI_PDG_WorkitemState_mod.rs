#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_PDG_WorkitemState {
    HAPI_PDG_WORKITEM_COOKED_CACHE = 6i32,
    HAPI_PDG_WORKITEM_COOKED_CANCEL = 8i32,
    HAPI_PDG_WORKITEM_COOKED_FAIL = 7i32,
    HAPI_PDG_WORKITEM_COOKED_SUCCESS = 5i32,
    HAPI_PDG_WORKITEM_COOKING = 4i32,
    HAPI_PDG_WORKITEM_DIRTY = 9i32,
    HAPI_PDG_WORKITEM_SCHEDULED = 3i32,
    HAPI_PDG_WORKITEM_UNCOOKED = 1i32,
    HAPI_PDG_WORKITEM_UNDEFINED = 0i32,
    HAPI_PDG_WORKITEM_WAITING = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_PDG_WorkitemState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_PDG_WorkitemState =>
    "HoudiniEngineUnity"."HAPI_PDG_WorkitemState"
);
