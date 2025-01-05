#[cfg(feature = "UnityEngine+UI+Button")]
#[repr(C)]
#[derive(Debug)]
pub struct Button {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    pub m_OnClick: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Button_ButtonClickedEvent,
    >,
}
#[cfg(feature = "UnityEngine+UI+Button")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Button => "UnityEngine.UI"
    ."Button"
);
#[cfg(feature = "UnityEngine+UI+Button")]
impl std::ops::Deref for crate::UnityEngine::UI::Button {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>;
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
    #[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
    pub type ButtonClickedEvent = crate::UnityEngine::UI::Button_ButtonClickedEvent;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinishSubmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("OnFinishSubmit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSubmit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Press(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Press", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button_ButtonClickedEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::Button_ButtonClickedEvent,
        > = __cordl_object.invoke("get_onClick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onClick(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::Button_ButtonClickedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onClick", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::UnityEngine::UI::Button {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::UnityEngine::UI::Button {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerClickHandler>,
> for crate::UnityEngine::UI::Button {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerClickHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerClickHandler>,
> for crate::UnityEngine::UI::Button {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerClickHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>>
for crate::UnityEngine::UI::Button {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>>
for crate::UnityEngine::UI::Button {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::ISubmitHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct Button_ButtonClickedEvent {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
}
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Button_ButtonClickedEvent =>
    "UnityEngine.UI"."Button/ButtonClickedEvent"
);
#[cfg(feature = "UnityEngine+UI+Button+ButtonClickedEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::Button_ButtonClickedEvent {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
