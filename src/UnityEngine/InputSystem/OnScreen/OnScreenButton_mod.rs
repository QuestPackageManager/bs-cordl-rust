#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
#[repr(C)]
#[derive(Debug)]
pub struct OnScreenButton {
    __cordl_parent: crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    pub m_ControlPath: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenButton =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenButton"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    type Target = crate::UnityEngine::InputSystem::OnScreen::OnScreenControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPointerDown(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
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
    pub fn get_controlPathInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_controlPathInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controlPathInternal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPathInternal", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerDownHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerDownHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenButton")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerUpHandler>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenButton {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerUpHandler {
        unsafe { std::mem::transmute(self) }
    }
}
