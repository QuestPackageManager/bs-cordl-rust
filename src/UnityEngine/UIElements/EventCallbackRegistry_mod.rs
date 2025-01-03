#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Callbacks: *mut crate::UnityEngine::UIElements::EventCallbackList,
    pub m_TemporaryCallbacks: *mut crate::UnityEngine::UIElements::EventCallbackList,
    pub m_IsInvoking: i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCallbackRegistry
    => "UnityEngine.UIElements"."EventCallbackRegistry"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCallbackList", (initializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCallbackListForReading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventCallbackList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = __cordl_object.invoke("GetCallbackListForReading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCallbackListForWriting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventCallbackList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        > = __cordl_object.invoke("GetCallbackListForWriting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasBubbleHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBubbleHandlers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTrickleDownHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTrickleDownHandlers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacks(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCallbacks", (evt, propagationPhase))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (callback, useTrickleDown, invokePolicy))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RegisterCallback",
                (callback, userArgs, useTrickleDown, invokePolicy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseCallbackList(
        toRelease: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallbackList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseCallbackList", (toRelease))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnregisterCallback", (callback, useTrickleDown))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback_i64_Delegate_TrickleDown0(
        &mut self,
        eventTypeId: i64,
        callback: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnregisterCallback", (eventTypeId, callback, useTrickleDown))?;
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
