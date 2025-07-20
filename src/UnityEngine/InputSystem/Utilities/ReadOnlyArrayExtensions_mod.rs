#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadOnlyArrayExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "ReadOnlyArrayExtensions";
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    pub fn Contains<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                                TValue,
                            >,
                            TValue,
                        ),
                        bool,
                        2usize,
                    >("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Contains", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                                TValue,
                            >,
                            TValue,
                        ),
                        bool,
                        2usize,
                    >("ContainsReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsReference", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HaveEqualReferences<TValue>(
        array1: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue>,
        array2: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<TValue>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                                TValue,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<TValue>,
                            >,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("HaveEqualReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HaveEqualReferences", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array1, array2, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                                TValue,
                            >,
                            TValue,
                        ),
                        i32,
                        2usize,
                    >("IndexOfReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexOfReference", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
