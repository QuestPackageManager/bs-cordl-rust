#[cfg(feature = "HoudiniEngineUnity+HAPI_RSTOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_RSTOrder {
    #[default]
    HAPI_RST = 3i32,
    HAPI_RSTORDER_DEFAULT = 5i32,
    HAPI_RTS = 2i32,
    HAPI_STR = 4i32,
    HAPI_TRS = 0i32,
    HAPI_TSR = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_RSTOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_RSTOrder =>
    "HoudiniEngineUnity"."HAPI_RSTOrder"
);
