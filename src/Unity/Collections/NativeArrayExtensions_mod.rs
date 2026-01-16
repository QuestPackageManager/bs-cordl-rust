#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeArrayExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::NativeArrayExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeArrayExtensions";
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
#[cfg(feature = "Unity+Collections+NativeArrayExtensions")]
impl std::ops::Deref for crate::Unity::Collections::NativeArrayExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::NativeArrayExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayExtensions")]
impl crate::Unity::Collections::NativeArrayExtensions {
    #[cfg(feature = "Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
    pub type NativeArrayStaticId_1<T: quest_hook::libil2cpp::Type> =
        crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>;
    pub fn ArraysEqual<T>(
        container: crate::Unity::Collections::NativeArray_1<T>,
        other: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<T>,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), bool, 2usize>("ArraysEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ArraysEqual",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckReinterpretSize<T, U>(
        array: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckReinterpretSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckReinterpretSize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Il2CppObject_i32_U2<T, U>(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
        value: U,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        U,
                    ), bool, 3usize>("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Contains",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ptr, length, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_NativeArray_1_ReadOnly_U1<T, U>(
        array: crate::Unity::Collections::NativeArray_1_ReadOnly<T>,
        value: U,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1_ReadOnly<T>, U),
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
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (array, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_NativeArray_1_U0<T, U>(
        array: crate::Unity::Collections::NativeArray_1<T>,
        value: U,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<T>, U),
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
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (array, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom_ByRefMut1<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        other: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom_ByRefMut2<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeHashSet_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::LowLevel::Unsafe::UnsafeHashSet_1<T>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom_NativeList_1_0<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        other: crate::Unity::Collections::NativeList_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyFrom",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeCheckAllocator<T>(
        array: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisposeCheckAllocator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisposeCheckAllocator", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppObject_i32_U2<T, U>(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
        value: U,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        U,
                    ), i32, 3usize>("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IndexOf",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ptr, length, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_NativeArray_1_ReadOnly_U1<T, U>(
        array: crate::Unity::Collections::NativeArray_1_ReadOnly<T>,
        value: U,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1_ReadOnly<T>, U),
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
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (array, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_NativeArray_1_U0<T, U>(
        array: crate::Unity::Collections::NativeArray_1<T>,
        value: U,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<T>, U),
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
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (array, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_AllocatorManager_AllocatorHandle0<T>(
        array: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        length: i32,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        options: crate::Unity::Collections::NativeArrayOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        i32,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        crate::Unity::Collections::NativeArrayOptions,
                    ), quest_hook::libil2cpp::Void, 4usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array, length, allocator, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_ByRefMut1<T, U>(
        array: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        length: i32,
        allocator: quest_hook::libil2cpp::ByRefMut<U>,
        options: crate::Unity::Collections::NativeArrayOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<U>,
                        crate::Unity::Collections::NativeArrayOptions,
                    ), quest_hook::libil2cpp::Void, 4usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (array, length, allocator, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn Reinterpret<T, U>(
        array: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<U>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type
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
                        crate::Unity::Collections::NativeArray_1<U>,
                        1usize,
                    >("Reinterpret")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Reinterpret", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<U> =
            unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::NativeArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeArrayExtensions_NativeArrayStaticId_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeArrayExtensions/NativeArrayStaticId`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Collections",
                "NativeArrayExtensions/NativeArrayStaticId`1",
            )
            .unwrap()
            .make_generic::<(T)>()
            .unwrap()
            .unwrap()
        })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayExtensions+NativeArrayStaticId_1")]
impl<T: quest_hook::libil2cpp::Type>
    crate::Unity::Collections::NativeArrayExtensions_NativeArrayStaticId_1<T>
{
}
