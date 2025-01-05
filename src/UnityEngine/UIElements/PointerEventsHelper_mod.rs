#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerEventsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerEventsHelper =>
    "UnityEngine.UIElements"."PointerEventsHelper"
);
#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerEventsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerEventsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
impl crate::UnityEngine::UIElements::PointerEventsHelper {
    pub fn SendEnterLeave<TLeaveEvent, TEnterEvent>(
        previousTopElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        currentTopElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPointerEvent,
        >,
        position: crate::UnityEngine::Vector2,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TLeaveEvent: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TEnterEvent: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SendEnterLeave",
                (
                    previousTopElementUnderPointer,
                    currentTopElementUnderPointer,
                    triggerEvent,
                    position,
                    pointerId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendOverOut(
        previousTopElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        currentTopElementUnderPointer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPointerEvent,
        >,
        position: crate::UnityEngine::Vector2,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SendOverOut",
                (
                    previousTopElementUnderPointer,
                    currentTopElementUnderPointer,
                    triggerEvent,
                    position,
                    pointerId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerEventsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerEventsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
