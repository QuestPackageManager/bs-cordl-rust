#[cfg(feature = "UnityEngine+UI+ColorBlock")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ColorBlock {
    pub m_NormalColor: crate::UnityEngine::Color,
    pub m_HighlightedColor: crate::UnityEngine::Color,
    pub m_PressedColor: crate::UnityEngine::Color,
    pub m_SelectedColor: crate::UnityEngine::Color,
    pub m_DisabledColor: crate::UnityEngine::Color,
    pub m_ColorMultiplier: f32,
    pub m_FadeDuration: f32,
}
#[cfg(feature = "UnityEngine+UI+ColorBlock")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ColorBlock => "UnityEngine.UI"
    ."ColorBlock"
);
#[cfg(feature = "UnityEngine+UI+ColorBlock")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UI::ColorBlock {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UI+ColorBlock")]
impl crate::UnityEngine::UI::ColorBlock {
    pub fn Equals_ColorBlock1(
        &mut self,
        other: crate::UnityEngine::UI::ColorBlock,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_colorMultiplier",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_disabledColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fadeDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fadeDuration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_highlightedColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normalColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressedColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectedColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_colorMultiplier",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disabledColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_disabledColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fadeDuration(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_fadeDuration",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highlightedColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_highlightedColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normalColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_normalColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pressedColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressedColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectedColor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ColorBlock")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UI::ColorBlock>>
for crate::UnityEngine::UI::ColorBlock {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UI::ColorBlock> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+ColorBlock")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UI::ColorBlock>>
for crate::UnityEngine::UI::ColorBlock {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UI::ColorBlock> {
        todo!()
    }
}
