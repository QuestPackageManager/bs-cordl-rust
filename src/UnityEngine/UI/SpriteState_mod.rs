#[cfg(feature = "UnityEngine+UI+SpriteState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpriteState {
    pub m_HighlightedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_PressedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_SelectedSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub m_DisabledSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
}
#[cfg(feature = "UnityEngine+UI+SpriteState")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::SpriteState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "SpriteState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UI::SpriteState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::UI::SpriteState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UI::SpriteState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UI::SpriteState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
