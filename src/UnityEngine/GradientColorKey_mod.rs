#[cfg(feature = "UnityEngine+GradientColorKey")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GradientColorKey {
    pub color: crate::UnityEngine::Color,
    pub _cordl_time: f32,
}
#[cfg(feature = "UnityEngine+GradientColorKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GradientColorKey => "UnityEngine"
    ."GradientColorKey"
);
#[cfg(feature = "UnityEngine+GradientColorKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::GradientColorKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+GradientColorKey")]
impl crate::UnityEngine::GradientColorKey {
    pub fn _ctor(
        &mut self,
        col: crate::UnityEngine::Color,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (col, _cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
}
