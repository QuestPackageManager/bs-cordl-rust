#[cfg(feature = "HoudiniEngineUnity+HAPI_StatusVerbosity")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_StatusVerbosity {
    #[default]
    HAPI_STATUSVERBOSITY_0 = 0i32,
    HAPI_STATUSVERBOSITY_1 = 1i32,
    HAPI_STATUSVERBOSITY_2 = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_StatusVerbosity")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_StatusVerbosity =>
    "HoudiniEngineUnity"."HAPI_StatusVerbosity"
);
