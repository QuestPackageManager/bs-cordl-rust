#[cfg(feature = "UnityEngine+DrivenRectTransformTracker")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DrivenRectTransformTracker {}
#[cfg(feature = "UnityEngine+DrivenRectTransformTracker")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DrivenRectTransformTracker =>
    "UnityEngine"."DrivenRectTransformTracker"
);
#[cfg(feature = "UnityEngine+DrivenRectTransformTracker")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::DrivenRectTransformTracker {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+DrivenRectTransformTracker")]
impl crate::UnityEngine::DrivenRectTransformTracker {
    pub fn Add(
        &mut self,
        driver: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        drivenProperties: crate::UnityEngine::DrivenTransformProperties,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (driver, rectTransform, drivenProperties),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
