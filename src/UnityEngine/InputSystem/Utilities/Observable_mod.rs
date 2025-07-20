#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
#[repr(C)]
#[derive(Debug)]
pub struct Observable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Observable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::Observable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "Observable";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                2usize,
            >("Call")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "Call", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked((), (source, action))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                2usize,
            >("CallOnce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "CallOnce", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked((), (source, action))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::IObservable_1<
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::IObservable_1<
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    >,
                >,
                1usize,
            >("ForDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "ForDevice", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        > = unsafe { method.invoke_unchecked((), (source))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::IObservable_1<
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::IObservable_1<
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    >,
                >,
                2usize,
            >("ForDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "ForDevice", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        > = unsafe { method.invoke_unchecked((), (source, device))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TResult>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TResult>>,
                2usize,
            >("Select")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "Select", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TResult>,
        > = unsafe { method.invoke_unchecked((), (source, filter))? };
        Ok(__cordl_ret.into())
    }
    pub fn SelectMany<TSource, TResult>(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TResult>,
                >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            TSource,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<TResult>,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TResult>>,
                2usize,
            >("SelectMany")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "SelectMany", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TResult>,
        > = unsafe { method.invoke_unchecked((), (source, filter))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>, i32),
                quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                2usize,
            >("Take")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "Take", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TValue>,
        > = unsafe { method.invoke_unchecked((), (source, count))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::Utilities::Observable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                2usize,
            >("Where")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::Utilities::Observable as
                    quest_hook::libil2cpp::Type > ::class(), "Where", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<TValue>,
        > = unsafe { method.invoke_unchecked((), (source, predicate))? };
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
