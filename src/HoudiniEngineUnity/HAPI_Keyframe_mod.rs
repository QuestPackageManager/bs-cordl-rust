#[cfg(feature = "HoudiniEngineUnity+HAPI_Keyframe")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HAPI_Keyframe {
    pub _cordl_time: f32,
    pub value: f32,
    pub inTangent: f32,
    pub outTangent: f32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Keyframe")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_Keyframe =>
    "HoudiniEngineUnity"."HAPI_Keyframe"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_Keyframe")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_Keyframe {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_Keyframe")]
impl crate::HoudiniEngineUnity::HAPI_Keyframe {
    pub fn _ctor(
        &mut self,
        t: f32,
        v: f32,
        in_tangent: f32,
        out_tangent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (t, v, in_tangent, out_tangent),
        )?;
        Ok(__cordl_ret.into())
    }
}
