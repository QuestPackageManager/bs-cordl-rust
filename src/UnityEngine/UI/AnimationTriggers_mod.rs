#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimationTriggers {
    __cordl_parent: crate::System::Object,
    pub m_NormalTrigger: *mut crate::System::String,
    pub m_HighlightedTrigger: *mut crate::System::String,
    pub m_PressedTrigger: *mut crate::System::String,
    pub m_SelectedTrigger: *mut crate::System::String,
    pub m_DisabledTrigger: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::AnimationTriggers =>
    "UnityEngine.UI"."AnimationTriggers"
);
#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
impl std::ops::Deref for crate::UnityEngine::UI::AnimationTriggers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
impl std::ops::DerefMut for crate::UnityEngine::UI::AnimationTriggers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
impl crate::UnityEngine::UI::AnimationTriggers {
    pub const kDefaultDisabledAnimName: &'static str = "Disabled";
    pub const kDefaultHighlightedAnimName: &'static str = "Highlighted";
    pub const kDefaultNormalAnimName: &'static str = "Normal";
    pub const kDefaultPressedAnimName: &'static str = "Pressed";
    pub const kDefaultSelectedAnimName: &'static str = "Selected";
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_disabledTrigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_disabledTrigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_highlightedTrigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_highlightedTrigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalTrigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_normalTrigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pressedTrigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_pressedTrigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedTrigger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_selectedTrigger", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disabledTrigger(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disabledTrigger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_highlightedTrigger(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highlightedTrigger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_normalTrigger(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normalTrigger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pressedTrigger(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pressedTrigger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectedTrigger(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedTrigger", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+AnimationTriggers")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::AnimationTriggers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
