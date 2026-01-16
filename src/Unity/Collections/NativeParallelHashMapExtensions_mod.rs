#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelHashMapExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeParallelHashMapExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelHashMapExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::NativeParallelHashMapExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeParallelHashMapExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+Collections+NativeParallelHashMapExtensions")]
impl std::ops::Deref for crate::Unity::Collections::NativeParallelHashMapExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeParallelHashMapExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeParallelHashMapExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeParallelHashMapExtensions")]
impl crate::Unity::Collections::NativeParallelHashMapExtensions {
    pub fn GetUniqueKeyArray_NativeParallelMultiHashMap_2_1<TKey, TValue>(
        container: crate::Unity::Collections::NativeParallelMultiHashMap_2<TKey, TValue>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<crate::Unity::Collections::NativeArray_1<TKey>, i32>,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeParallelMultiHashMap_2<
                                TKey,
                                TValue,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::System::ValueTuple_2<
                            crate::Unity::Collections::NativeArray_1<TKey>,
                            i32,
                        >,
                        2usize,
                    >("GetUniqueKeyArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUniqueKeyArray", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::Unity::Collections::NativeArray_1<TKey>,
            i32,
        > = unsafe { cordl_method_info.invoke_unchecked((), (container, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueKeyArray_UnsafeParallelMultiHashMap_2_0<TKey, TValue>(
        container: crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
            TKey,
            TValue,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<crate::Unity::Collections::NativeArray_1<TKey>, i32>,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
                                TKey,
                                TValue,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::System::ValueTuple_2<
                            crate::Unity::Collections::NativeArray_1<TKey>,
                            i32,
                        >,
                        2usize,
                    >("GetUniqueKeyArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUniqueKeyArray", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::Unity::Collections::NativeArray_1<TKey>,
            i32,
        > = unsafe { cordl_method_info.invoke_unchecked((), (container, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeBucketData_NativeParallelHashMap_2_0<TKey, TValue>(
        container: crate::Unity::Collections::NativeParallelHashMap_2<TKey, TValue>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeParallelHashMap_2<
                            TKey,
                            TValue,
                        >),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData,
                        1usize,
                    >("GetUnsafeBucketData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafeBucketData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData = unsafe {
            cordl_method_info.invoke_unchecked((), (container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeBucketData_NativeParallelMultiHashMap_2_1<TKey, TValue>(
        container: crate::Unity::Collections::NativeParallelMultiHashMap_2<TKey, TValue>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeParallelMultiHashMap_2<
                            TKey,
                            TValue,
                        >),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData,
                        1usize,
                    >("GetUnsafeBucketData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafeBucketData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelHashMapBucketData = unsafe {
            cordl_method_info.invoke_unchecked((), (container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Remove<TKey, TValue>(
        container: crate::Unity::Collections::NativeParallelMultiHashMap_2<TKey, TValue>,
        key: TKey,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeParallelMultiHashMap_2<TKey, TValue>,
                        TKey,
                        TValue,
                    ), quest_hook::libil2cpp::Void, 3usize>("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Remove",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, key, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Unique<T>(
        array: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<T>),
                        i32,
                        1usize,
                    >("Unique")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Unique",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeParallelHashMapExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::NativeParallelHashMapExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
