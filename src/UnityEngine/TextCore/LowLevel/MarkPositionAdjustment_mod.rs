#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkPositionAdjustment")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MarkPositionAdjustment {
    pub m_XPositionAdjustment: f32,
    pub m_YPositionAdjustment: f32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkPositionAdjustment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment =>
    "UnityEngine.TextCore.LowLevel"."MarkPositionAdjustment"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkPositionAdjustment")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+MarkPositionAdjustment")]
impl crate::UnityEngine::TextCore::LowLevel::MarkPositionAdjustment {
    pub fn get_xPositionAdjustment(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xPositionAdjustment",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yPositionAdjustment(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yPositionAdjustment",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
