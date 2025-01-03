#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
#[repr(C)]
#[derive(Debug)]
pub struct EventCallbackFunctor_2<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::EventCallbackFunctorBase,
    pub m_Callback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs>,
    >,
    pub m_EventTypeId: i64,
    pub _userArgs_k__BackingField: TCallbackArgs,
    __cordl_phantom_TEventType: std::marker::PhantomData<TEventType>,
    __cordl_phantom_TCallbackArgs: std::marker::PhantomData<TCallbackArgs>,
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventCallbackFunctor_2
    < TEventType, TCallbackArgs > => "UnityEngine.UIElements"."EventCallbackFunctor`2" <
    TEventType, TCallbackArgs >
);
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::EventCallbackFunctor_2<TEventType, TCallbackArgs> {
    type Target = crate::UnityEngine::UIElements::EventCallbackFunctorBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::EventCallbackFunctor_2<TEventType, TCallbackArgs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::EventCallbackFunctor_2<TEventType, TCallbackArgs> {
    pub fn Invoke(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        propagationPhase: crate::UnityEngine::UIElements::PropagationPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (evt, propagationPhase))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        eventTypeId: i64,
        callback: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEquivalentTo", (eventTypeId, callback, phase))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs>,
        >,
        userArgs: TCallbackArgs,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, userArgs, phase, invokePolicy))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventCallback_2<TEventType, TCallbackArgs>,
        >,
        userArgs: TCallbackArgs,
        phase: crate::UnityEngine::UIElements::CallbackPhase,
        invokePolicy: crate::UnityEngine::UIElements::InvokePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, userArgs, phase, invokePolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userArgs(&mut self) -> quest_hook::libil2cpp::Result<TCallbackArgs>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TCallbackArgs = __cordl_object.invoke("get_userArgs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_userArgs(
        &mut self,
        value: TCallbackArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCallbackArgs: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userArgs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCallbackFunctor_2")]
impl<
    TEventType: quest_hook::libil2cpp::Type,
    TCallbackArgs: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventCallbackFunctor_2<TEventType, TCallbackArgs> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
