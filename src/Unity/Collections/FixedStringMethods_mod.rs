#[cfg(feature = "cordl_class_Unity+Collections+FixedStringMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedStringMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::FixedStringMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "FixedStringMethods";
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
#[cfg(feature = "Unity+Collections+FixedStringMethods")]
impl std::ops::Deref for crate::Unity::Collections::FixedStringMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedStringMethods")]
impl std::ops::DerefMut for crate::Unity::Collections::FixedStringMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedStringMethods")]
impl crate::Unity::Collections::FixedStringMethods {
    pub fn AppendFormat_ByRefMut1<T, U, T0, T1>(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                        ),
                        crate::Unity::Collections::FormatError,
                        4usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (dest, format, arg0, arg1))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut2<T, U, T0, T1, T2>(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        crate::Unity::Collections::FormatError,
                        5usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (dest, format, arg0, arg1, arg2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut0<T, U, T0>(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                        ),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (dest, format, arg0))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut3<T, U, T0, T1, T2, T3>(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                        ),
                        crate::Unity::Collections::FormatError,
                        6usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dest, format, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut4<T, U, T0, T1, T2, T3, T4>(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                        ),
                        crate::Unity::Collections::FormatError,
                        7usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dest, format, arg0, arg1, arg2, arg3, arg4))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut5<
        T,
        U,
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
    >(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
        arg5: quest_hook::libil2cpp::ByRefMut<T5>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                            quest_hook::libil2cpp::ByRefMut<T5>,
                        ),
                        crate::Unity::Collections::FormatError,
                        8usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 8usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, format, arg0, arg1, arg2, arg3, arg4, arg5),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut6<
        T,
        U,
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
    >(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
        arg5: quest_hook::libil2cpp::ByRefMut<T5>,
        arg6: quest_hook::libil2cpp::ByRefMut<T6>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                            quest_hook::libil2cpp::ByRefMut<T5>,
                            quest_hook::libil2cpp::ByRefMut<T6>,
                        ),
                        crate::Unity::Collections::FormatError,
                        9usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 9usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, format, arg0, arg1, arg2, arg3, arg4, arg5, arg6),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut7<
        T,
        U,
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
    >(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
        arg5: quest_hook::libil2cpp::ByRefMut<T5>,
        arg6: quest_hook::libil2cpp::ByRefMut<T6>,
        arg7: quest_hook::libil2cpp::ByRefMut<T7>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                            quest_hook::libil2cpp::ByRefMut<T5>,
                            quest_hook::libil2cpp::ByRefMut<T6>,
                            quest_hook::libil2cpp::ByRefMut<T7>,
                        ),
                        crate::Unity::Collections::FormatError,
                        10usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 10usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, format, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut8<
        T,
        U,
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
    >(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
        arg5: quest_hook::libil2cpp::ByRefMut<T5>,
        arg6: quest_hook::libil2cpp::ByRefMut<T6>,
        arg7: quest_hook::libil2cpp::ByRefMut<T7>,
        arg8: quest_hook::libil2cpp::ByRefMut<T8>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                            quest_hook::libil2cpp::ByRefMut<T5>,
                            quest_hook::libil2cpp::ByRefMut<T6>,
                            quest_hook::libil2cpp::ByRefMut<T7>,
                            quest_hook::libil2cpp::ByRefMut<T8>,
                        ),
                        crate::Unity::Collections::FormatError,
                        11usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 11usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, format, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendFormat_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut9<
        T,
        U,
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
    >(
        dest: quest_hook::libil2cpp::ByRefMut<T>,
        format: quest_hook::libil2cpp::ByRefMut<U>,
        arg0: quest_hook::libil2cpp::ByRefMut<T0>,
        arg1: quest_hook::libil2cpp::ByRefMut<T1>,
        arg2: quest_hook::libil2cpp::ByRefMut<T2>,
        arg3: quest_hook::libil2cpp::ByRefMut<T3>,
        arg4: quest_hook::libil2cpp::ByRefMut<T4>,
        arg5: quest_hook::libil2cpp::ByRefMut<T5>,
        arg6: quest_hook::libil2cpp::ByRefMut<T6>,
        arg7: quest_hook::libil2cpp::ByRefMut<T7>,
        arg8: quest_hook::libil2cpp::ByRefMut<T8>,
        arg9: quest_hook::libil2cpp::ByRefMut<T9>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T9: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                            quest_hook::libil2cpp::ByRefMut<T0>,
                            quest_hook::libil2cpp::ByRefMut<T1>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            quest_hook::libil2cpp::ByRefMut<T3>,
                            quest_hook::libil2cpp::ByRefMut<T4>,
                            quest_hook::libil2cpp::ByRefMut<T5>,
                            quest_hook::libil2cpp::ByRefMut<T6>,
                            quest_hook::libil2cpp::ByRefMut<T7>,
                            quest_hook::libil2cpp::ByRefMut<T8>,
                            quest_hook::libil2cpp::ByRefMut<T9>,
                        ),
                        crate::Unity::Collections::FormatError,
                        12usize,
                    >("AppendFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendFormat", 12usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        dest,
                        format,
                        arg0,
                        arg1,
                        arg2,
                        arg3,
                        arg4,
                        arg5,
                        arg6,
                        arg7,
                        arg8,
                        arg9,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendRawByte<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        a: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, u8),
                        crate::Unity::Collections::FormatError,
                        2usize,
                    >("AppendRawByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendRawByte", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, a))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppendScientific<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sourceLength: i32,
        decimalExponent: i32,
        decimalSeparator: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            i32,
                            char,
                        ),
                        crate::Unity::Collections::FormatError,
                        5usize,
                    >("AppendScientific")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendScientific", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (fs, source, sourceLength, decimalExponent, decimalSeparator),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_ByRefMut8<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_Il2CppObject_i32_9<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        utf8Bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8BytesLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, utf8Bytes, utf8BytesLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_Il2CppString10<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_Unicode_Rune0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
                        ),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, rune))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_Unicode_Rune_i32_2<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
                            i32,
                        ),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, rune, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, char),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, ch))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char__cordl_char11<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        a: char,
        b: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, char, char),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char__cordl_char__cordl_char12<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        a: char,
        b: char,
        c: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, char, char, char),
                        crate::Unity::Collections::FormatError,
                        4usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, a, b, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char__cordl_char__cordl_char__cordl_char__cordl_char__cordl_char__cordl_char__cordl_char13<
        T,
    >(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        a: char,
        b: char,
        c: char,
        d: char,
        e: char,
        f: char,
        g: char,
        h: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                        ),
                        crate::Unity::Collections::FormatError,
                        9usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            9usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, a, b, c, d, e, f, g, h))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_f32__cordl_char7<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: f32,
        decimalSeparator: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, f32, char),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input, decimalSeparator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_i32_4<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i32),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_i64_3<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i64),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_u32_6<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, u32),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_u64_5<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, u64),
                        crate::Unity::Collections::FormatError,
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
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSubstringInRange(
        strLength: i32,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CheckSubstringInRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckSubstringInRange", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (strLength, startIndex, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_ByRefMut1<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        i32,
                        2usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject_i32_0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, bytes, bytesLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeHashCode<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        i32,
                        1usize,
                    >("ComputeHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeHashCode", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
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
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToString<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ConvertToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromTruncated_ByRefMut1<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        crate::Unity::Collections::CopyError,
                        2usize,
                    >("CopyFromTruncated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFromTruncated", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromTruncated_Il2CppString0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::Unity::Collections::CopyError,
                        2usize,
                    >("CopyFromTruncated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFromTruncated", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom_ByRefMut0<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        input: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        crate::Unity::Collections::CopyError,
                        2usize,
                    >("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFrom", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom_Il2CppString1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::Unity::Collections::CopyError,
                        2usize,
                    >("CopyFrom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyFrom", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, s))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EffectiveSizeOf<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        i32,
                        1usize,
                    >("EffectiveSizeOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EffectiveSizeOf", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_ByRefMut1<T, U>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<U>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                        ),
                        bool,
                        2usize,
                    >("EndsWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndsWith", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_Unicode_Rune0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
                        ),
                        bool,
                        2usize,
                    >("EndsWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndsWith", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, rune))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ByRefMut1<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        bool,
                        2usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject_i32_0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, bytes, bytesLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Found_ByRefMut_ByRefMut__cordl_char__cordl_char__cordl_char0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        a: char,
        b: char,
        c: char,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            char,
                            char,
                            char,
                        ),
                        bool,
                        5usize,
                    >("Found")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Found",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, offset, a, b, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Found__cordl_char__cordl_char__cordl_char__cordl_char__cordl_char1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        a: char,
        b: char,
        c: char,
        d: char,
        e: char,
        f: char,
        g: char,
        h: char,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                            char,
                        ),
                        bool,
                        10usize,
                    >("Found")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Found",
                            10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, offset, a, b, c, d, e, f, g, h))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_ByRefMut3<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
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
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_ByRefMut_i32_i32_4<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
        startIndex: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
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
            cordl_method_info.invoke_unchecked((), (fs, other, startIndex, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppObject_i32_1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, bytes, bytesLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppObject_i32_i32_i32_2<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
        startIndex: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        i32,
                        5usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IndexOf",
                            5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (fs, bytes, bytesLen, startIndex, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Unicode_Rune0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
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
            cordl_method_info.invoke_unchecked((), (fs, rune))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_ByRefMut3<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                        ),
                        i32,
                        2usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndexOf", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_ByRefMut_i32_i32_4<T, T2>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<T2>,
        startIndex: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T2>,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndexOf", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other, startIndex, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppObject_i32_1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndexOf", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, bytes, bytesLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppObject_i32_i32_i32_2<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bytesLen: i32,
        startIndex: i32,
        distance: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        i32,
                        5usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndexOf", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (fs, bytes, bytesLen, startIndex, distance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Unicode_Rune0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
                        ),
                        i32,
                        2usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LastIndexOf", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, rune))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseLongInternal<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        bool,
                        3usize,
                    >("ParseLongInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseLongInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, offset, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ByRefMut_ByRefMut_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        output: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::ParseError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        crate::Unity::Collections::ParseError,
                        3usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::ParseError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, offset, output))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ByRefMut_ByRefMut_ByRefMut1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        output: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::ParseError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::Unity::Collections::ParseError,
                        3usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::ParseError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, offset, output))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse__cordl_char2<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        output: quest_hook::libil2cpp::ByRefMut<f32>,
        decimalSeparator: char,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::ParseError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            char,
                        ),
                        crate::Unity::Collections::ParseError,
                        4usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::ParseError = unsafe {
            cordl_method_info
                .invoke_unchecked((), (fs, offset, output, decimalSeparator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Peek<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::Unicode_Rune>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i32),
                        crate::Unity::Collections::Unicode_Rune,
                        2usize,
                    >("Peek")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Peek",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::Unicode_Rune = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::Unicode_Rune>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        crate::Unity::Collections::Unicode_Rune,
                        2usize,
                    >("Read")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Read",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::Unicode_Rune = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith_ByRefMut1<T, U>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        other: quest_hook::libil2cpp::ByRefMut<U>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<U>,
                        ),
                        bool,
                        2usize,
                    >("StartsWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartsWith", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith_Unicode_Rune0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::Unity::Collections::Unicode_Rune,
                        ),
                        bool,
                        2usize,
                    >("StartsWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartsWith", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, rune))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_AllocatorManager_AllocatorHandle3(
        str: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        startIndex: i32,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            i32,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        3usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_ByRefMut_i32_1<T>(
        str: quest_hook::libil2cpp::ByRefMut<T>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i32),
                        T,
                        2usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_ByRefMut_i32_5(
        str: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_i32_0<T>(
        str: quest_hook::libil2cpp::ByRefMut<T>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, i32, i32),
                        T,
                        3usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 3usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_i32_4(
        str: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            i32,
                            i32,
                        ),
                        crate::Unity::Collections::NativeText,
                        3usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Substring_i32_AllocatorManager_AllocatorHandle2(
        str: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        startIndex: i32,
        length: i32,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            i32,
                            i32,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        4usize,
                    >("Substring")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Substring", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (str, startIndex, length, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerAscii_AllocatorManager_AllocatorHandle1(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        2usize,
                    >("ToLowerAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLowerAscii", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerAscii_AllocatorManager_AllocatorHandle2(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("ToLowerAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLowerAscii", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLowerAscii_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("ToLowerAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLowerAscii", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperAscii_AllocatorManager_AllocatorHandle1(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        2usize,
                    >("ToUpperAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToUpperAscii", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperAscii_AllocatorManager_AllocatorHandle2(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("ToUpperAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToUpperAscii", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperAscii_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("ToUpperAscii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToUpperAscii", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndIndex_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        i32,
                        1usize,
                    >("TrimEndIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimEndIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEndIndex_ReadOnlySpan_1_1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        i32,
                        2usize,
                    >("TrimEndIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimEndIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_AllocatorManager_AllocatorHandle1(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        2usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_AllocatorManager_AllocatorHandle2(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_4(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        3usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_5(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::NativeText,
                        3usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd_ReadOnlySpan_1_3<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        T,
                        2usize,
                    >("TrimEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "TrimEnd",
                            2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStartIndex_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        i32,
                        1usize,
                    >("TrimStartIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStartIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStartIndex_ReadOnlySpan_1_1<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        i32,
                        2usize,
                    >("TrimStartIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStartIndex", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_AllocatorManager_AllocatorHandle1(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        2usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_AllocatorManager_AllocatorHandle2(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_4(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        3usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_5(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::NativeText,
                        3usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart_ReadOnlySpan_1_3<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        T,
                        2usize,
                    >("TrimStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrimStart", 2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_AllocatorManager_AllocatorHandle1(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        2usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_AllocatorManager_AllocatorHandle2(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                        ),
                        crate::Unity::Collections::NativeText,
                        2usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_4(
        fs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
        >,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::LowLevel::Unsafe::UnsafeText,
                        3usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::LowLevel::Unsafe::UnsafeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_AllocatorManager_AllocatorHandle_ReadOnlySpan_1_5(
        fs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeText,
                            >,
                            crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        crate::Unity::Collections::NativeText,
                        3usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, allocator, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_ByRefMut0<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>),
                        T,
                        1usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked((), (fs))? };
        Ok(__cordl_ret.into())
    }
    pub fn Trim_ReadOnlySpan_1_3<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        trimRunes: crate::System::ReadOnlySpan_1<crate::Unity::Collections::Unicode_Rune>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            crate::System::ReadOnlySpan_1<
                                crate::Unity::Collections::Unicode_Rune,
                            >,
                        ),
                        T,
                        2usize,
                    >("Trim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Trim",
                            2usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, trimRunes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write<T>(
        fs: quest_hook::libil2cpp::ByRefMut<T>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        rune: crate::Unity::Collections::Unicode_Rune,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            crate::Unity::Collections::Unicode_Rune,
                        ),
                        crate::Unity::Collections::FormatError,
                        3usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info.invoke_unchecked((), (fs, index, rune))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedStringMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::FixedStringMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
