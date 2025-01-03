#[cfg(feature = "UnityEngine+Keyframe")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Keyframe {
    pub m_Time: f32,
    pub m_Value: f32,
    pub m_InTangent: f32,
    pub m_OutTangent: f32,
    pub m_WeightedMode: i32,
    pub m_InWeight: f32,
    pub m_OutWeight: f32,
}
#[cfg(feature = "UnityEngine+Keyframe")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Keyframe => "UnityEngine"
    ."Keyframe"
);
#[cfg(feature = "UnityEngine+Keyframe")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Keyframe {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Keyframe")]
impl crate::UnityEngine::Keyframe {
    pub fn _ctor_f32_f32_0(
        &mut self,
        _cordl_time: f32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_1(
        &mut self,
        _cordl_time: f32,
        value: f32,
        inTangent: f32,
        outTangent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time, value, inTangent, outTangent),
        )?;
        Ok(__cordl_ret.into())
    }
}
