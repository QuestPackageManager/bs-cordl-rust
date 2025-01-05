#[cfg(feature = "HMUI+HoverTextSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct HoverTextSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _hoverTextController: quest_hook::libil2cpp::Gc<
        crate::HMUI::HoverTextController,
    >,
    pub _text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HMUI+HoverTextSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::HoverTextSetter => "HMUI"
    ."HoverTextSetter"
);
#[cfg(feature = "HMUI+HoverTextSetter")]
impl std::ops::Deref for crate::HMUI::HoverTextSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl std::ops::DerefMut for crate::HMUI::HoverTextSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl crate::HMUI::HoverTextSetter {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
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
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_text", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_text(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::HoverTextSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::HoverTextSetter {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::HoverTextSetter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::HoverTextSetter {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::HoverTextSetter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::HoverTextSetter {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+HoverTextSetter")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::HMUI::HoverTextSetter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
