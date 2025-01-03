#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerEventHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerEventHelper =>
    "UnityEngine.UIElements"."PointerEventHelper"
);
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerEventHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerEventHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl crate::UnityEngine::UIElements::PointerEventHelper {
    pub fn GetPooled(
        eventType: crate::UnityEngine::EventType,
        mousePosition: crate::UnityEngine::Vector3,
        delta: crate::UnityEngine::Vector2,
        button: i32,
        clickCount: i32,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetPooled",
                (eventType, mousePosition, delta, button, clickCount, modifiers),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerEventHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
