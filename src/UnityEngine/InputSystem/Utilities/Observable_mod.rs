#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
#[repr(C)]
#[derive(Debug)]
pub struct Observable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::Observable
    => "UnityEngine.InputSystem.Utilities"."Observable"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::Observable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::Observable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl crate::UnityEngine::InputSystem::Utilities::Observable {
    pub fn Call<TValue>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Call", (source, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallOnce<TValue>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallOnce", (source, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForDevice_IObservable_1_1<TDevice>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    >
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForDevice", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForDevice_InputDevice0(
        source: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForDevice", (source, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn Select<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select", (source, filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectMany<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                *mut crate::System::Collections::Generic::IEnumerable_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TResult>>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectMany", (source, filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn Take<TValue>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Take", (source, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Where<TValue>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Where", (source, predicate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::Observable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
