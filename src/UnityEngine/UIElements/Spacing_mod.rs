#[cfg(feature = "UnityEngine+UIElements+Spacing")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Spacing {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[cfg(feature = "UnityEngine+UIElements+Spacing")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Spacing =>
    "UnityEngine.UIElements"."Spacing"
);
#[cfg(feature = "UnityEngine+UIElements+Spacing")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Spacing {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Spacing")]
impl crate::UnityEngine::UIElements::Spacing {
    pub fn _ctor(
        &mut self,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (left, top, right, bottom),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontal(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_horizontal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertical(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertical",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        r: crate::UnityEngine::Rect,
        a: crate::UnityEngine::UIElements::Spacing,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (r, a))?;
        Ok(__cordl_ret.into())
    }
}
