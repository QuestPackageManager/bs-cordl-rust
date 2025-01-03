#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseEventsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseEventsHelper =>
    "UnityEngine.UIElements"."MouseEventsHelper"
);
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseEventsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseEventsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl crate::UnityEngine::UIElements::MouseEventsHelper {
    pub fn SendEnterLeave<TLeaveEvent, TEnterEvent>(
        previousTopElementUnderMouse: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        currentTopElementUnderMouse: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IMouseEvent,
        >,
        mousePosition: crate::UnityEngine::Vector2,
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
                    previousTopElementUnderMouse,
                    currentTopElementUnderMouse,
                    triggerEvent,
                    mousePosition,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendMouseOverMouseOut(
        previousTopElementUnderMouse: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        currentTopElementUnderMouse: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IMouseEvent,
        >,
        mousePosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SendMouseOverMouseOut",
                (
                    previousTopElementUnderMouse,
                    currentTopElementUnderMouse,
                    triggerEvent,
                    mousePosition,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseEventsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
