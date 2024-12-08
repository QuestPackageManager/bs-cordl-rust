#[cfg(feature = "HoudiniEngineUnity+HAPI_ChoiceListType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAPI_ChoiceListType {
    HAPI_CHOICELISTTYPE_MINI = 2i32,
    HAPI_CHOICELISTTYPE_NONE = 0i32,
    HAPI_CHOICELISTTYPE_NORMAL = 1i32,
    HAPI_CHOICELISTTYPE_REPLACE = 3i32,
    HAPI_CHOICELISTTYPE_TOGGLE = 4i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_ChoiceListType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_ChoiceListType =>
    "HoudiniEngineUnity"."HAPI_ChoiceListType"
);
