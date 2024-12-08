#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectManyObservable_2_Select<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub m_Observable: *mut crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
        TSource,
        TResult,
    >,
    pub m_Observer: *mut crate::System::IObserver_1<TResult>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select < TSource,
    TResult > => "UnityEngine.InputSystem.Utilities"."SelectManyObservable`2/Select" <
    TSource, TResult >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
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
        Ok(__cordl_ret)
    }
    pub fn OnError(
        &mut self,
        error: *mut crate::System::Exception,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        observable: *mut crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
            TSource,
            TResult,
        >,
        observer: *mut crate::System::IObserver_1<TResult>,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        observable: *mut crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
            TSource,
            TResult,
        >,
        observer: *mut crate::System::IObserver_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (observable, observer))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectManyObservable_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub m_Source: *mut crate::System::IObservable_1<TSource>,
    pub m_Filter: *mut crate::System::Func_2<
        TSource,
        *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
    >,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::SelectManyObservable_2 < TSource, TResult > =>
    "UnityEngine.InputSystem.Utilities"."SelectManyObservable`2" < TSource, TResult >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<TSource, TResult> {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
    pub type Select = crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
        TSource,
        TResult,
    >;
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::IObservable_1<TSource>,
        filter: *mut crate::System::Func_2<
            TSource,
            *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
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
        Ok(__cordl_ret)
    }
    pub fn Subscribe(
        &mut self,
        observer: *mut crate::System::IObserver_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IDisposable>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IDisposable = __cordl_object
            .invoke("Subscribe", (observer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        source: *mut crate::System::IObservable_1<TSource>,
        filter: *mut crate::System::Func_2<
            TSource,
            *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, filter))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
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
