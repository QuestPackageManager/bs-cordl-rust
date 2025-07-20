#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Callbacks: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallbackList,
    >,
    pub m_TemporaryCallbacks: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallbackList,
    >,
    pub m_IsInvoking: i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::EventCallbackRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventCallbackRegistry";
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
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventCallbackRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventCallbackRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
impl crate::UnityEngine::UIElements::EventCallbackRegistry {
    pub fn GetCallbackList(
        initializer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventCallbackList>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::EventCallbackList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::EventCallbackList,
                >,
                1usize,
            >("GetCallbackList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "GetCallbackList", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = unsafe { method.invoke_unchecked((), (initializer))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCallbackListForReading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventCallbackList>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::EventCallbackList,
                >,
                0usize,
            >("GetCallbackListForReading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "GetCallbackListForReading",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCallbackListForWriting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventCallbackList>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::EventCallbackList,
                >,
                0usize,
            >("GetCallbackListForWriting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "GetCallbackListForWriting",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasBubbleHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasBubbleHandlers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "HasBubbleHandlers", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasTrickleDownHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasTrickleDownHandlers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "HasTrickleDownHandlers",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacks(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
                    crate::UnityEngine::UIElements::PropagationPhase,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InvokeCallbacks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "InvokeCallbacks", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt, propagationPhase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterCallback_EventCallback_1_TrickleDown_InvokePolicy0<TEventType>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        >,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
                    >,
                    crate::UnityEngine::UIElements::TrickleDown,
                    crate::UnityEngine::UIElements::InvokePolicy,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RegisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterCallback", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback, useTrickleDown, invokePolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_EventCallback_2_TCallbackArgs_TrickleDown_InvokePolicy1<
        TEventType,
        TCallbackArgs,
    >(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs>,
        >,
        userArgs: TCallbackArgs,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::EventCallback_2<
                            TEventType,
                            TCallbackArgs,
                        >,
                    >,
                    TCallbackArgs,
                    crate::UnityEngine::UIElements::TrickleDown,
                    crate::UnityEngine::UIElements::InvokePolicy,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("RegisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterCallback", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (callback, userArgs, useTrickleDown, invokePolicy),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseCallbackList(
        toRelease: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::EventCallbackList,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReleaseCallbackList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "ReleaseCallbackList",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (toRelease))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback_EventCallback_1_TrickleDown1<TEventType>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        >,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
                    >,
                    crate::UnityEngine::UIElements::TrickleDown,
                ),
                bool,
                2usize,
            >("UnregisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "UnregisterCallback", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (callback, useTrickleDown))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback_i64_Delegate_TrickleDown0(
        &mut self,
        eventTypeId: i64,
        callback: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i64,
                    quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                    crate::UnityEngine::UIElements::TrickleDown,
                ),
                bool,
                3usize,
            >("UnregisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), "UnregisterCallback", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (eventTypeId, callback, useTrickleDown))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::EventCallbackRegistry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::EventCallbackRegistry as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallbackRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
