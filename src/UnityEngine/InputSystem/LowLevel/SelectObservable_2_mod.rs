#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectObservable_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
    pub m_Filter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::SelectObservable_2 < TSource, TResult > =>
    "UnityEngine.InputSystem.LowLevel"."SelectObservable`2" < TSource, TResult >
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
    pub type Select = crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
        TSource,
        TResult,
    >;
    pub fn New(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, filter))?;
        Ok(__cordl_object.into())
    }
    pub fn Subscribe(
        &mut self,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = __cordl_object
            .invoke("Subscribe", (observer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, filter))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IObservable_1<TResult>>
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    fn as_ref(&self) -> &crate::System::IObservable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IObservable_1<TResult>>
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult> {
    fn as_mut(&mut self) -> &mut crate::System::IObservable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectObservable_2_Select<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Observable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<TSource, TResult>,
    >,
    pub m_Observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select < TSource, TResult >
    => "UnityEngine.InputSystem.LowLevel"."SelectObservable`2/Select" < TSource, TResult
    >
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    pub fn New(
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<
                TSource,
                TResult,
            >,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (observable, observer))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        evt: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2<
                TSource,
                TResult,
            >,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (observable, observer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IObserver_1<TSource>>
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_ref(&self) -> &crate::System::IObserver_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SelectObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IObserver_1<TSource>>
for crate::UnityEngine::InputSystem::LowLevel::SelectObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_mut(&mut self) -> &mut crate::System::IObserver_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
