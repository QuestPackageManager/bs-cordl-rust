#[cfg(feature = "UnityEngine+UIElements+StyleSelectorPart")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleSelectorPart {
    pub m_Value: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Type: crate::UnityEngine::UIElements::StyleSelectorType,
    pub tempData: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelectorPart")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSelectorPart =>
    "UnityEngine.UIElements"."StyleSelectorPart"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSelectorPart")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSelectorPart {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSelectorPart")]
impl crate::UnityEngine::UIElements::StyleSelectorPart {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSelectorType,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSelectorType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_type(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSelectorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_type",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
