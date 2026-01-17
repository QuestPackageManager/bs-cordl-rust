#[cfg(feature = "cordl_class_Unity+Collections+HashSetExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct HashSetExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+HashSetExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::HashSetExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "HashSetExtensions";
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
#[cfg(feature = "Unity+Collections+HashSetExtensions")]
impl std::ops::Deref for crate::Unity::Collections::HashSetExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+HashSetExtensions")]
impl std::ops::DerefMut for crate::Unity::Collections::HashSetExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+HashSetExtensions")]
impl crate::Unity::Collections::HashSetExtensions {
    pub fn ExceptWith_FixedList128Bytes_1_0<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList128Bytes_1_11<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList32Bytes_1_1<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList32Bytes_1_12<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList4096Bytes_1_13<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList4096Bytes_1_2<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList512Bytes_1_14<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList512Bytes_1_3<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList64Bytes_1_15<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_FixedList64Bytes_1_4<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeArray_1_16<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeArray_1_5<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeHashSet_1_17<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeHashSet_1_6<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeHashSet_1_ReadOnly18<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeHashSet_1_ReadOnly7<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeList_1_10<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeList_1_21<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeParallelHashSet_1_19<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeParallelHashSet_1_8<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeParallelHashSet_1_ReadOnly20<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExceptWith_NativeParallelHashSet_1_ReadOnly9<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ExceptWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExceptWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList128Bytes_1_0<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList128Bytes_1_11<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList32Bytes_1_1<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList32Bytes_1_12<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList4096Bytes_1_13<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList4096Bytes_1_2<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList512Bytes_1_14<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList512Bytes_1_3<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList64Bytes_1_15<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_FixedList64Bytes_1_4<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeArray_1_16<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeArray_1_5<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeHashSet_1_17<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeHashSet_1_6<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeHashSet_1_ReadOnly18<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeHashSet_1_ReadOnly7<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeList_1_10<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeList_1_21<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeParallelHashSet_1_19<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeParallelHashSet_1_8<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeParallelHashSet_1_ReadOnly20<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectWith_NativeParallelHashSet_1_ReadOnly9<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("IntersectWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IntersectWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList128Bytes_1_0<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList128Bytes_1_11<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList128Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList128Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList32Bytes_1_1<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList32Bytes_1_12<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList32Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList32Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList4096Bytes_1_13<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList4096Bytes_1_2<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList4096Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList4096Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList512Bytes_1_14<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList512Bytes_1_3<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList512Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList512Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList64Bytes_1_15<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_FixedList64Bytes_1_4<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::FixedList64Bytes_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::FixedList64Bytes_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeArray_1_16<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeArray_1_5<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeArray_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeHashSet_1_17<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeHashSet_1_6<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeHashSet_1_ReadOnly18<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeHashSet_1_ReadOnly7<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeList_1_10<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeList_1_21<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeList_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeParallelHashSet_1_19<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeParallelHashSet_1_8<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeParallelHashSet_1_ReadOnly20<T>(
        container: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeParallelHashSet_1<T>,
        >,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeParallelHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnionWith_NativeParallelHashSet_1_ReadOnly9<T>(
        container: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeHashSet_1<T>>,
        other: crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
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
                            crate::Unity::Collections::NativeHashSet_1<T>,
                        >,
                        crate::Unity::Collections::NativeParallelHashSet_1_ReadOnly<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("UnionWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnionWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (container, other))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+HashSetExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::HashSetExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
