#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerEventDataExtension {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PointerEventDataExtension => "UnityEngine.EventSystems"
    ."PointerEventDataExtension"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl crate::UnityEngine::EventSystems::PointerEventDataExtension {
    pub fn GetRay(
        pointerEventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_ret: crate::UnityEngine::Ray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRay", (pointerEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSwipeStart(
        pointerEventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSwipeStart", (pointerEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVRPointer(
        pointerEventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVRPointer", (pointerEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSwipeStart(
        pointerEventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        start: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSwipeStart", (pointerEventData, start))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerEventDataExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerEventDataExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
