#[cfg(feature = "UnityEngine+UIElements+StyleValueHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleValueHandle {
    pub m_ValueType: crate::UnityEngine::UIElements::StyleValueType,
    pub valueIndex: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueHandle =>
    "UnityEngine.UIElements"."StyleValueHandle"
);
#[cfg(feature = "UnityEngine+UIElements+StyleValueHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleValueHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueHandle")]
impl crate::UnityEngine::UIElements::StyleValueHandle {
    pub fn get_valueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleValueType> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleValueType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valueType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_valueType(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleValueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_valueType",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
