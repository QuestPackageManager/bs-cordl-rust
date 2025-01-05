#[cfg(feature = "UnityEngine+UI+SpriteState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SpriteState {
    pub m_HighlightedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_PressedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_SelectedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_DisabledSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
}
#[cfg(feature = "UnityEngine+UI+SpriteState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::SpriteState => "UnityEngine.UI"
    ."SpriteState"
);
#[cfg(feature = "UnityEngine+UI+SpriteState")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UI::SpriteState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UI+SpriteState")]
impl crate::UnityEngine::UI::SpriteState {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::UI::SpriteState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_disabledSprite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highlightedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_highlightedSprite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pressedSprite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedSprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectedSprite",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disabledSprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_disabledSprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highlightedSprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_highlightedSprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pressedSprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pressedSprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedSprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectedSprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+SpriteState")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UI::SpriteState>>
for crate::UnityEngine::UI::SpriteState {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UI::SpriteState> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+SpriteState")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UI::SpriteState>>
for crate::UnityEngine::UI::SpriteState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UI::SpriteState> {
        todo!()
    }
}
