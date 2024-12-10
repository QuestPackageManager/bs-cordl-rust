#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbackEventHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub isIMGUIContainer: bool,
    pub m_CallbackRegistry: *mut crate::UnityEngine::UIElements::EventCallbackRegistry,
}
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::CallbackEventHandler =>
    "UnityEngine.UIElements"."CallbackEventHandler"
);
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::CallbackEventHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::CallbackEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl crate::UnityEngine::UIElements::CallbackEventHandler {
    pub const ExecuteDefaultActionAtTargetName: &'static str = "ExecuteDefaultActionAtTarget";
    pub const ExecuteDefaultActionName: &'static str = "ExecuteDefaultAction";
    pub fn AddEventCategories<TEventType>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEventCategories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultAction(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultAction", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultActionAtTarget(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionAtTarget", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultActionDisabled(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionDisabled", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultActionDisabledAtTarget(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionDisabledAtTarget", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEventAtCurrentTargetAndPhase(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEventAtCurrentTargetAndPhase", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEventAtTargetAndDefaultPhase(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEventAtTargetAndDefaultPhase", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEventAtTargetPhase(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEventAtTargetPhase", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasBubbleUpHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBubbleUpHandlers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTrickleDownHandlers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTrickleDownHandlers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterCallback_EventCallback_1_InvokePolicy_TrickleDown2<TEventType>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        >,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (callback, invokePolicy, useTrickleDown))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_EventCallback_1_TrickleDown0<TEventType>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        >,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (callback, useTrickleDown))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_EventCallback_2_TUserArgsType_TrickleDown1<
        TEventType,
        TUserArgsType,
    >(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_2<TEventType, TUserArgsType>,
        >,
        userArgs: TUserArgsType,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TUserArgsType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (callback, userArgs, useTrickleDown))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent_DispatchMode1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (e, dispatchMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent_EventBase0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IEventHandler_HandleEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IEventHandler.HandleEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback<TEventType>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        >,
        useTrickleDown: crate::UnityEngine::UIElements::TrickleDown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallback", (callback, useTrickleDown))?;
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
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::CallbackEventHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl AsRef<crate::UnityEngine::UIElements::IEventHandler>
for crate::UnityEngine::UIElements::CallbackEventHandler {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IEventHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CallbackEventHandler")]
impl AsMut<crate::UnityEngine::UIElements::IEventHandler>
for crate::UnityEngine::UIElements::CallbackEventHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IEventHandler {
        unsafe { std::mem::transmute(self) }
    }
}
