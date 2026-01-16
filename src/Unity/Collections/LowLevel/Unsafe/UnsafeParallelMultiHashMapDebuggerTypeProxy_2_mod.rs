#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Target:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<TKey, TValue>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2"
)]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
        TKey,
        TValue,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "UnsafeParallelMultiHashMapDebuggerTypeProxy`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Collections.LowLevel.Unsafe",
                "UnsafeParallelMultiHashMapDebuggerTypeProxy`2",
            )
            .unwrap()
            .make_generic::<(TKey, TValue)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
        TKey,
        TValue,
    >
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
        TKey,
        TValue,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
        TKey,
        TValue,
    >
{
    pub fn GetUniqueKeyArray(
        hashMap: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<TKey, TValue>,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<crate::Unity::Collections::NativeArray_1<TKey>, i32>,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
                                    TKey,
                                    TValue,
                                >,
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
        > = unsafe { cordl_method_info.invoke_unchecked((), (hashMap, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        target: crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
            TKey,
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (target))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        target: crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
            TKey,
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMap_2<
                            TKey,
                            TValue,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (target))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Unity::Collections::ListPair_2<
                    TKey,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TValue>>,
                >,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Unity::Collections::ListPair_2<
                                TKey,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<TValue>,
                                >,
                            >,
                        >,
                    >, 0usize>("get_Items")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Items",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Unity::Collections::ListPair_2<
                    TKey,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TValue>>,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+LowLevel+Unsafe+UnsafeParallelMultiHashMapDebuggerTypeProxy_2"
)]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::LowLevel::Unsafe::UnsafeParallelMultiHashMapDebuggerTypeProxy_2<
        TKey,
        TValue,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
