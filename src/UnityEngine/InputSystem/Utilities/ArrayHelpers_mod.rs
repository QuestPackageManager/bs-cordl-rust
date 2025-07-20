#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "ArrayHelpers";
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    pub fn AppendListWithCapacity<TValue, TValues>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        values: TValues,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValues: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            TValues,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("AppendListWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendListWithCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, length, values, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendToImmutable<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                        ),
                        i32,
                        2usize,
                    >("AppendToImmutable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendToImmutable", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendWithCapacity_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        value: TValue,
        capacityIncrement: i32,
        allocator: crate::Unity::Collections::Allocator,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            TValue,
                            i32,
                            crate::Unity::Collections::Allocator,
                        ),
                        i32,
                        5usize,
                    >("AppendWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendWithCapacity", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (array, count, value, capacityIncrement, allocator),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendWithCapacity_ByRefMut_ByRefMut_TValue_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        value: TValue,
        capacityIncrement: i32,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            TValue,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("AppendWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendWithCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, count, value, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_IEnumerable_1_1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TValue>,
        >,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<TValue>,
                            >,
                        ),
                        i32,
                        2usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_TValue0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            TValue,
                        ),
                        i32,
                        2usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear_ByRefMut2<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear_i32_1<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clone<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >,
                        1usize,
                    >("Clone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clone",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
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
    pub fn ContainsReference_TValue0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
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
    pub fn ContainsReference_i32_TSecond1<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        count: i32,
        value: TSecond,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TFirst>,
                            >,
                            i32,
                            TSecond,
                        ),
                        bool,
                        3usize,
                    >("ContainsReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsReference", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference_i32_i32_TSecond2<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        startIndex: i32,
        count: i32,
        value: TSecond,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TFirst>,
                            >,
                            i32,
                            i32,
                            TSecond,
                        ),
                        bool,
                        4usize,
                    >("ContainsReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContainsReference", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array, startIndex, count, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >,
                        1usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: i32,
        capacity: i32,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DuplicateWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DuplicateWithCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, count, capacity, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: i32,
        capacity: i32,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("EnsureCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, count, capacity, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Erase<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            TValue,
                        ),
                        bool,
                        2usize,
                    >("Erase")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Erase",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAt<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EraseAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "EraseAt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtByMovingTail<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EraseAtByMovingTail")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EraseAtByMovingTail", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtWithCapacity_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EraseAtWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EraseAtWithCapacity", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAtWithCapacity_NativeArray_1_1<TValue>(
        array: crate::Unity::Collections::NativeArray_1<TValue>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            crate::Unity::Collections::NativeArray_1<TValue>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("EraseAtWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EraseAtWithCapacity", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EraseSliceWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("EraseSliceWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EraseSliceWithCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, length, index, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GrowBy_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: i32,
        allocator: crate::Unity::Collections::Allocator,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<TValue>,
                            >,
                            i32,
                            crate::Unity::Collections::Allocator,
                        ),
                        i32,
                        3usize,
                    >("GrowBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GrowBy",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GrowBy_ByRefMut_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: i32,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                        ),
                        i32,
                        2usize,
                    >("GrowBy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GrowBy",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GrowWithCapacity_Allocator1<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        growBy: i32,
        capacityIncrement: i32,
        allocator: crate::Unity::Collections::Allocator,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<TValue>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            crate::Unity::Collections::Allocator,
                        ),
                        i32,
                        5usize,
                    >("GrowWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GrowWithCapacity", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (array, count, growBy, capacityIncrement, allocator),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GrowWithCapacity_ByRefMut_ByRefMut_i32_i32_0<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        growBy: i32,
        capacityIncrement: i32,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("GrowWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GrowWithCapacity", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, count, growBy, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HaveDuplicateReferences<TFirst>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TFirst>,
                            >,
                            i32,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("HaveDuplicateReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HaveDuplicateReferences", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (first, index, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HaveEqualElements<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("HaveEqualElements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HaveEqualElements", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference_Il2CppArray_TSecond_i32_0<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        value: TSecond,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TFirst>,
                            >,
                            TSecond,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("IndexOfReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexOfReference", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference_i32_1<TFirst, TSecond>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TFirst>>,
        value: TSecond,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TFirst>,
                            >,
                            TSecond,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexOfReference", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value, startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfValue<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
        startIndex: i32,
        count: i32,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            TValue,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IndexOfValue", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value, startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Predicate_1_1<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
                        ),
                        i32,
                        2usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Predicate_1_i32_i32_2<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
        startIndex: i32,
        count: i32,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Predicate_1<TValue>,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, predicate, startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_TValue_i32_i32_0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        value: TValue,
        startIndex: i32,
        count: i32,
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            TValue,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array, value, startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertAt<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        index: i32,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                            TValue,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InsertAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertAt", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, index, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertAtWithCapacity<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
        index: i32,
        value: TValue,
        capacityIncrement: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            TValue,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InsertAtWithCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertAtWithCapacity", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, count, index, value, capacityIncrement))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Join<TValue>(
        value: TValue,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
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
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >,
                        2usize,
                    >("Join")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Join",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (value, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn LengthSafe<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
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
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >),
                        i32,
                        1usize,
                    >("LengthSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LengthSafe", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (array))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Merge_IEqualityComparer_1_1<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEqualityComparer_1<
                                    TValue,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >,
                        3usize,
                    >("Merge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Merge",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Merge_Il2CppArray_Il2CppArray0<TValue>(
        first: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        second: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TValue>,
                        >,
                        2usize,
                    >("Merge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Merge",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (first, second))? };
        Ok(__cordl_ret.into())
    }
    pub fn MoveSlice<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        sourceIndex: i32,
        destinationIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("MoveSlice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveSlice", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (array, sourceIndex, destinationIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PutAtIfNotSet<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        >,
        index: i32,
        valueFn: quest_hook::libil2cpp::Gc<crate::System::Func_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<TValue>,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<TValue>>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PutAtIfNotSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PutAtIfNotSet", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, index, valueFn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Resize<TValue>(
        array: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<TValue>,
        >,
        newSize: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<TValue>,
                            >,
                            i32,
                            crate::Unity::Collections::Allocator,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Resize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Resize",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, newSize, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Select<TOld, TNew>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TOld>>,
        converter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TOld, TNew>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TNew>>,
    >
    where
        TOld: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TNew: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TOld>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_2<TOld, TNew>>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<TNew>,
                        >,
                        2usize,
                    >("Select")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Select",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TNew>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (array, converter))? };
        Ok(__cordl_ret.into())
    }
    pub fn Swap<TValue>(
        first: quest_hook::libil2cpp::ByRefMut<TValue>,
        second: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Swap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Swap",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (first, second))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements_Il2CppArray0<TValue>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<TValue>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SwapElements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SwapElements", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, index1, index2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwapElements_NativeArray_1_1<TValue>(
        array: crate::Unity::Collections::NativeArray_1<TValue>,
        index1: i32,
        index2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<TValue>, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SwapElements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SwapElements", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (array, index1, index2))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ArrayHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ArrayHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
