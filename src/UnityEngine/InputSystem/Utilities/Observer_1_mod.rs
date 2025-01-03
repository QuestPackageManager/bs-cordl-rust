#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Observer_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_OnNext: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    pub m_OnCompleted: quest_hook::libil2cpp::Gc<crate::System::Action>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::Observer_1
    < TValue > => "UnityEngine.InputSystem.Utilities"."Observer`1" < TValue >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    pub fn New(
        onNext: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
        onCompleted: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (onNext, onCompleted))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnError(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnError", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNext(
        &mut self,
        evt: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNext", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        onNext: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
        onCompleted: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (onNext, onCompleted))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::IObserver_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    fn as_ref(&self) -> &crate::System::IObserver_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observer_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::IObserver_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::Observer_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::IObserver_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
