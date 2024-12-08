#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackFunctor_1<TEventType: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::EventCallbackFunctorBase,
    pub m_Callback: *mut crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
    pub m_EventTypeId: i64,
    __cordl_phantom_TEventType: std::marker::PhantomData<TEventType>,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCallbackFunctor_1
    < TEventType > => "UnityEngine.UIElements"."EventCallbackFunctor`1" < TEventType >
);
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
impl<TEventType: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::EventCallbackFunctor_1<TEventType> {
    type Target = crate::UnityEngine::UIElements::EventCallbackFunctorBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
impl<TEventType: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::EventCallbackFunctor_1<TEventType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::EventCallbackFunctor_1<TEventType> {
    pub fn Invoke(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (evt, propagationPhase))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        callback: *mut crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, phase, invokePolicy))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        eventTypeId: i64,
        callback: *mut crate::System::Delegate,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEquivalentTo", (eventTypeId, callback, phase))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        callback: *mut crate::UnityEngine::UIElements::EventCallback_1<TEventType>,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, phase, invokePolicy))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_1")]
impl<TEventType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallbackFunctor_1<TEventType> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
