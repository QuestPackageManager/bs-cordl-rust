#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseEventsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MouseEventsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "MouseEventsHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseEventsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseEventsHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::IMouseEvent,
                            >,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SendEnterLeave")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendEnterLeave", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        previousTopElementUnderMouse,
                        currentTopElementUnderMouse,
                        triggerEvent,
                        mousePosition,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::IMouseEvent,
                            >,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SendMouseOverMouseOut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendMouseOverMouseOut", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        previousTopElementUnderMouse,
                        currentTopElementUnderMouse,
                        triggerEvent,
                        mousePosition,
                    ),
                )?
        };
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
