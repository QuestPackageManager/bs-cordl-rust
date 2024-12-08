#[cfg(feature = "UnityEngine+UI+Button")]
#[repr(C)]
#[derive(Debug)]
pub struct Button {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub m_OnClick: *mut crate::UnityEngine::UI::Button_ButtonClickedEvent,
}
#[cfg(feature = "UnityEngine+UI+Button")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Button => "UnityEngine.UI"
    ."Button"
);
#[cfg(feature = "UnityEngine+UI+Button")]
impl std::ops::Deref for crate::UnityEngine::UI::Button {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl crate::UnityEngine::UI::Button {
    #[cfg(feature = "UnityEngine+UI+Button+_OnFinishSubmit_d__9")]
    pub type _OnFinishSubmit_d__9 = crate::UnityEngine::UI::Button__OnFinishSubmit_d__9;
    #[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
    pub type ButtonClickedEvent = crate::UnityEngine::UI::Button_ButtonClickedEvent;
    pub fn OnPointerClick(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn Press(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Press", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onClick(
        &mut self,
        value: *mut crate::UnityEngine::UI::Button_ButtonClickedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onClick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_onClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::Button_ButtonClickedEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button_ButtonClickedEvent = __cordl_object
            .invoke("get_onClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnFinishSubmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("OnFinishSubmit", ())?;
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
    pub fn OnSubmit(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Button {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct Button_ButtonClickedEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent,
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Button_ButtonClickedEvent =>
    "UnityEngine.UI"."Button/ButtonClickedEvent"
);
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::Button_ButtonClickedEvent {
    type Target = crate::UnityEngine::Events::UnityEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Button_ButtonClickedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
impl crate::UnityEngine::UI::Button_ButtonClickedEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::Button_ButtonClickedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
