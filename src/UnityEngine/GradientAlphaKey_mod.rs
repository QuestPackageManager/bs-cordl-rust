#[cfg(feature = "UnityEngine+GradientAlphaKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GradientAlphaKey {
    pub alpha: f32,
    pub _cordl_time: f32,
}
#[cfg(feature = "UnityEngine+GradientAlphaKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GradientAlphaKey => "UnityEngine"
    ."GradientAlphaKey"
);
#[cfg(feature = "UnityEngine+GradientAlphaKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::GradientAlphaKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+GradientAlphaKey")]
impl crate::UnityEngine::GradientAlphaKey {
    pub fn _ctor(
        &mut self,
        alpha: f32,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (alpha, _cordl_time),
        )?;
        Ok(__cordl_ret)
    }
}
