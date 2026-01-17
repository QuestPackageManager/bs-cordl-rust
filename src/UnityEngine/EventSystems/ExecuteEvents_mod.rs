#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ExecuteEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::EventSystems::ExecuteEvents {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "ExecuteEvents";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::ExecuteEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents")]
impl crate::UnityEngine::EventSystems::ExecuteEvents {
    #[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
    pub type EventFunction_1<T1: quest_hook::libil2cpp::Type> =
        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>;
    pub fn CanHandleEvent<T>(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        bool,
                        1usize,
                    >("CanHandleEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanHandleEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (go))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteHierarchy<T>(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
        callbackFunction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T>,
                        >,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>, 3usize>(
                        "ExecuteHierarchy",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteHierarchy",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (root, eventData, callbackFunction))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_GameObject_ExecuteEvents_EventFunction_1_18<T>(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
        functor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T>,
                        >,
                    ), bool, 3usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (target, eventData, functor))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IBeginDragHandler7(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IBeginDragHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IBeginDragHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_ICancelHandler17(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IDeselectHandler14(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDeselectHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IDeselectHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IDragHandler8(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IDropHandler10(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IEndDragHandler9(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEndDragHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IEndDragHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IInitializePotentialDragHandler6(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IMoveHandler15(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerClickHandler5(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerClickHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerClickHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerDownHandler3(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerDownHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerDownHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerEnterHandler1(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerEnterHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerEnterHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerExitHandler2(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerExitHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerExitHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerMoveHandler0(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerMoveHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerMoveHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IPointerUpHandler4(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerUpHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IPointerUpHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IScrollHandler11(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_ISelectHandler13(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_ISubmitHandler16(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_IUpdateSelectedHandler12(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
        >,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventChain(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        eventChain: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("GetEventChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEventChain",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (root, eventChain))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventHandler<T>(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        1usize,
                    >("GetEventHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEventHandler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (root))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventList<T>(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IList_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::EventSystems::IEventSystemHandler,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("GetEventList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEventList",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (go, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendToComponent<T>(
        component: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>),
                        bool,
                        1usize,
                    >("ShouldSendToComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldSendToComponent", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (component))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateEventData<T>(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::EventSystems::BaseEventData,
                        >),
                        T,
                        1usize,
                    >("ValidateEventData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateEventData", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_beginDragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IBeginDragHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IBeginDragHandler,
                            >,
                        >,
                    >, 0usize>("get_beginDragHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_beginDragHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IBeginDragHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cancelHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::ICancelHandler,
                            >,
                        >,
                    >, 0usize>("get_cancelHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_cancelHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ICancelHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_deselectHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDeselectHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IDeselectHandler,
                            >,
                        >,
                    >, 0usize>("get_deselectHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_deselectHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDeselectHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_dragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IDragHandler,
                            >,
                        >,
                    >, 0usize>("get_dragHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_dragHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_dropHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IDropHandler,
                            >,
                        >,
                    >, 0usize>("get_dropHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_dropHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDropHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_endDragHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEndDragHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IEndDragHandler,
                            >,
                        >,
                    >, 0usize>("get_endDragHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_endDragHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEndDragHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_initializePotentialDrag() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
                            >,
                        >,
                    >, 0usize>("get_initializePotentialDrag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_initializePotentialDrag",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::IInitializePotentialDragHandler,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_moveHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IMoveHandler,
                            >,
                        >,
                    >, 0usize>("get_moveHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_moveHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IMoveHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerClickHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerClickHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerClickHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerClickHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerClickHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerClickHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerDownHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerDownHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerDownHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerDownHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerDownHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerDownHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerEnterHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerEnterHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerEnterHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerEnterHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerEnterHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerEnterHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerExitHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerExitHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerExitHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerExitHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerExitHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerExitHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerMoveHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerMoveHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerMoveHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerMoveHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerMoveHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerMoveHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pointerUpHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerUpHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IPointerUpHandler,
                            >,
                        >,
                    >, 0usize>("get_pointerUpHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_pointerUpHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerUpHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IScrollHandler,
                            >,
                        >,
                    >, 0usize>("get_scrollHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_scrollHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IScrollHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::ISelectHandler,
                            >,
                        >,
                    >, 0usize>("get_selectHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_selectHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISelectHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_submitHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::ISubmitHandler,
                            >,
                        >,
                    >, 0usize>("get_submitHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_submitHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::ISubmitHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_updateSelectedHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IUpdateSelectedHandler>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::IUpdateSelectedHandler,
                            >,
                        >,
                    >, 0usize>("get_updateSelectedHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_updateSelectedHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IUpdateSelectedHandler>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::EventSystems::ExecuteEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ExecuteEvents_EventFunction_1<T1: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "ExecuteEvents/EventFunction`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.EventSystems",
                "ExecuteEvents/EventFunction`1",
            )
            .unwrap()
            .make_generic::<(T1)>()
            .unwrap()
            .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>
{
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type>
    crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>
{
    pub fn BeginInvoke(
        &mut self,
        handler: T1,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        T1,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                        quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, 4usize>(
                        "BeginInvoke",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginInvoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked(self, (handler, eventData, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        handler: T1,
        eventData: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        T1,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseEventData>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handler, eventData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+EventSystems+ExecuteEvents+EventFunction_1")]
impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::EventSystems::ExecuteEvents_EventFunction_1<T1>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
