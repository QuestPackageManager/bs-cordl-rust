#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
#[repr(C)]
#[derive(Debug)]
pub struct ButtonSpriteSwapToggle {
    __cordl_parent: crate::HMUI::ButtonSpriteSwap,
    pub _resetToggleOnEnable: bool,
    pub _ignoreHighlight: bool,
    pub _isToggled: bool,
}
#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ButtonSpriteSwapToggle => "HMUI"
    ."ButtonSpriteSwapToggle"
);
#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
impl std::ops::Deref for crate::HMUI::ButtonSpriteSwapToggle {
    type Target = crate::HMUI::ButtonSpriteSwap;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
impl std::ops::DerefMut for crate::HMUI::ButtonSpriteSwapToggle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
impl crate::HMUI::ButtonSpriteSwapToggle {
    pub fn HandleButtonSelectionStateDidChange(
        &mut self,
        state: crate::HMUI::NoTransitionsButton_SelectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleButtonSelectionStateDidChange", (state))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isToggled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isToggled", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isToggled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isToggled", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+ButtonSpriteSwapToggle")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ButtonSpriteSwapToggle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
