#[cfg(feature = "HoudiniEngineUnity+HAPI_HeightFieldSampling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_HeightFieldSampling {
    #[default]
    HAPI_HEIGHTFIELD_SAMPLING_CENTER = 0i32,
    HAPI_HEIGHTFIELD_SAMPLING_CORNER = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_HeightFieldSampling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_HeightFieldSampling =>
    "HoudiniEngineUnity"."HAPI_HeightFieldSampling"
);
