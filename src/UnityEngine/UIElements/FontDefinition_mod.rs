#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontDefinition {
    pub m_Font: *mut crate::UnityEngine::Font,
    pub m_FontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FontDefinition =>
    "UnityEngine.UIElements"."FontDefinition"
);
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::FontDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FontDefinition")]
impl crate::UnityEngine::UIElements::FontDefinition {
    pub fn Equals_FontDefinition0(
        &mut self,
        other: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEmpty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_font(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Font> {
        let __cordl_ret: *mut crate::UnityEngine::Font = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_font",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_fontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    > {
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::FontAsset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fontAsset",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
