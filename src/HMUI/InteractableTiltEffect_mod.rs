#[cfg(feature = "HMUI+InteractableTiltEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct InteractableTiltEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rectTransform: *mut crate::UnityEngine::RectTransform,
    pub _maxHorizontalRotation: f32,
    pub _maxVerticalRotation: f32,
    pub _prevLocalPoint: crate::UnityEngine::Vector2,
    pub _effectStrengthMultiplier: f32,
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InteractableTiltEffect => "HMUI"
    ."InteractableTiltEffect"
);
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl std::ops::Deref for crate::HMUI::InteractableTiltEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl std::ops::DerefMut for crate::HMUI::InteractableTiltEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl crate::HMUI::InteractableTiltEffect {
    pub fn ComputeNewTargetRotation(
        &mut self,
        localPoint: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("ComputeNewTargetRotation", (localPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn OnPointerMove(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (eventData))?;
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
    pub fn get_effectStrengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_effectStrengthMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_effectStrengthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_effectStrengthMultiplier", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::InteractableTiltEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerMoveHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerMoveHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+InteractableTiltEffect")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerMoveHandler>
for crate::HMUI::InteractableTiltEffect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerMoveHandler {
        unsafe { std::mem::transmute(self) }
    }
}
