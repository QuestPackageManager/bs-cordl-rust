#[cfg(feature = "cordl_class_Unity+Collections+FixedString")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct FixedString {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedString")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::FixedString {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "FixedString";
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
#[cfg(feature = "Unity+Collections+FixedString")]
impl std::ops::Deref for crate::Unity::Collections::FixedString {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedString")]
impl std::ops::DerefMut for crate::Unity::Collections::FixedString {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+FixedString")]
impl crate::Unity::Collections::FixedString {
    pub fn Format_FixedString128Bytes_Il2CppString338(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 2usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_Il2CppString330(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_Il2CppString_Il2CppString298(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_Il2CppString_T1_314<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_Il2CppString_f32_282(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_Il2CppString_i32_266(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_T1_334<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_T1_Il2CppString302<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_T1_T2_318<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_T1_f32_286<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_T1_i32_270<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_f32_326(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_f32_Il2CppString294(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_f32_T1_310<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_f32_f32_278(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_f32_i32_262(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_i32_322(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_i32_Il2CppString290(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_i32_T1_306<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_i32_f32_274(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_Il2CppString_i32_i32_258(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_339<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        2usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_Il2CppString331<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_Il2CppString_Il2CppString299<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_Il2CppString_T2_315<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_Il2CppString_f32_283<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_Il2CppString_i32_267<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_T2_335<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, T2),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_T2_Il2CppString303<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: T2,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_T2_T3_319<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: T2,
        arg2: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, T2, T3),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_T2_f32_287<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: T2,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, T2, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_T2_i32_271<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: T2,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, T2, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_f32_327<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_f32_Il2CppString295<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_f32_T2_311<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: f32,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, f32, T2),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_f32_f32_279<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: f32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, f32, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_f32_i32_263<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: f32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, f32, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_i32_323<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_i32_Il2CppString291<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_i32_T2_307<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: i32,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, i32, T2),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_i32_f32_275<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: i32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, i32, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_T1_i32_i32_259<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: T1,
        arg1: i32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, T1, i32, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_337(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        2usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_Il2CppString329(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_Il2CppString_Il2CppString297(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_Il2CppString_T1_313<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_Il2CppString_f32_281(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_Il2CppString_i32_265(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_T1_333<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_T1_Il2CppString301<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_T1_T2_317<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: T1,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, T1, T2),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_T1_f32_285<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: T1,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, T1, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_T1_i32_269<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: T1,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, T1, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_f32_325(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_f32_Il2CppString293(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_f32_T1_309<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: f32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, f32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_f32_f32_277(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: f32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_f32_i32_261(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: f32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_i32_321(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_i32_Il2CppString289(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_i32_T1_305<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: i32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, f32, i32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_i32_f32_273(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: i32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_f32_i32_i32_257(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: f32,
        arg1: i32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        f32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_336(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        2usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_Il2CppString328(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 3usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_Il2CppString_Il2CppString296(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_Il2CppString_T1_312<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_Il2CppString_f32_280(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_Il2CppString_i32_264(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_T1_332<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_T1_Il2CppString300<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_T1_T2_316<T1, T2>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: T1,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, T1, T2),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_T1_f32_284<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: T1,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, T1, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_T1_i32_268<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: T1,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, T1, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_f32_324(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, f32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_f32_Il2CppString292(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_f32_T1_308<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: f32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, f32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_f32_f32_276(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: f32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_f32_i32_260(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: f32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_i32_320(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, i32),
                        crate::Unity::Collections::FixedString128Bytes,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_i32_Il2CppString288(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_i32_T1_304<T1>(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: i32,
        arg2: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::FixedString128Bytes, i32, i32, T1),
                        crate::Unity::Collections::FixedString128Bytes,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_i32_f32_272(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: i32,
        arg2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString128Bytes_i32_i32_i32_256(
        formatString: crate::Unity::Collections::FixedString128Bytes,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        i32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString128Bytes, 4usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2))? };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_Il2CppString_Il2CppString170(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_Il2CppString_T1_234<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_Il2CppString_f32_106(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_Il2CppString_i32_42(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_T1_Il2CppString186<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_T1_T2_250<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_T1_f32_122<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_T1_i32_58<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_f32_Il2CppString154(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_f32_T1_218<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_f32_f32_90(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_f32_i32_26(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_i32_Il2CppString138(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_i32_T1_202<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_i32_f32_74(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_Il2CppString_i32_i32_10(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_Il2CppString_Il2CppString174<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_Il2CppString_T2_238<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_Il2CppString_f32_110<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_Il2CppString_i32_46<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_T2_Il2CppString190<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_T2_T3_254<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_T2_f32_126<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_T2_i32_62<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_f32_Il2CppString158<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_f32_T2_222<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_f32_f32_94<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_f32_i32_30<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_i32_Il2CppString142<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_i32_T2_206<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_i32_f32_78<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_T1_i32_i32_14<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: T1,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_Il2CppString_Il2CppString166(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_Il2CppString_T1_230<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_Il2CppString_f32_102(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_Il2CppString_i32_38(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_T1_Il2CppString182<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_T1_T2_246<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_T1_f32_118<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_T1_i32_54<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_f32_Il2CppString150(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_f32_T1_214<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_f32_f32_86(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_f32_i32_22(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_i32_Il2CppString134(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_i32_T1_198<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_i32_f32_70(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_f32_i32_i32_6(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: f32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_Il2CppString_Il2CppString162(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_Il2CppString_T1_226<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_Il2CppString_f32_98(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_Il2CppString_i32_34(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_T1_Il2CppString178<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_T1_T2_242<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_T1_f32_114<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_T1_i32_50<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_f32_Il2CppString146(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_f32_T1_210<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_f32_f32_82(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_f32_i32_18(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_i32_Il2CppString130(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_i32_T1_194<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_i32_f32_66(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_Il2CppString_i32_i32_i32_2(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_Il2CppString_Il2CppString171<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_Il2CppString_T2_235<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_Il2CppString_f32_107<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_Il2CppString_i32_43<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_T2_Il2CppString187<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_T2_T3_251<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_T2_f32_123<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_T2_i32_59<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_f32_Il2CppString155<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_f32_T2_219<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_f32_f32_91<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_f32_i32_27<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_i32_Il2CppString139<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_i32_T2_203<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_i32_f32_75<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_Il2CppString_i32_i32_11<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_Il2CppString_Il2CppString175<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_Il2CppString_T3_239<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_Il2CppString_f32_111<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_Il2CppString_i32_47<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_T3_Il2CppString191<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: T3,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        T3,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_T3_T4_255<T1, T2, T3, T4>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: T3,
        arg3: T4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        T3,
                        T4,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_T3_f32_127<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: T3,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        T3,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_T3_i32_63<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: T3,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        T3,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_f32_Il2CppString159<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_f32_T3_223<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: f32,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        f32,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_f32_f32_95<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_f32_i32_31<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_i32_Il2CppString143<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_i32_T3_207<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: i32,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        i32,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_i32_f32_79<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_T2_i32_i32_15<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: T2,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        T2,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_Il2CppString_Il2CppString167<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_Il2CppString_T2_231<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_Il2CppString_f32_103<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_Il2CppString_i32_39<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_T2_Il2CppString183<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_T2_T3_247<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_T2_f32_119<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_T2_i32_55<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_f32_Il2CppString151<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_f32_T2_215<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_f32_f32_87<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_f32_i32_23<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_i32_Il2CppString135<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_i32_T2_199<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_i32_f32_71<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_f32_i32_i32_7<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: f32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        f32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_Il2CppString_Il2CppString163<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_Il2CppString_T2_227<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_Il2CppString_f32_99<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_Il2CppString_i32_35<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_T2_Il2CppString179<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_T2_T3_243<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_T2_f32_115<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_T2_i32_51<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_f32_Il2CppString147<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_f32_T2_211<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_f32_f32_83<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_f32_i32_19<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_i32_Il2CppString131<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_i32_T2_195<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_i32_f32_67<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_T1_i32_i32_i32_3<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: T1,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        T1,
                        i32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_Il2CppString_Il2CppString169(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_Il2CppString_T1_233<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_Il2CppString_f32_105(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_Il2CppString_i32_41(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_T1_Il2CppString185<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_T1_T2_249<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_T1_f32_121<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_T1_i32_57<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_f32_Il2CppString153(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_f32_T1_217<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_f32_f32_89(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_f32_i32_25(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_i32_Il2CppString137(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_i32_T1_201<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_i32_f32_73(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_Il2CppString_i32_i32_9(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_Il2CppString_Il2CppString173<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_Il2CppString_T2_237<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_Il2CppString_f32_109<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_Il2CppString_i32_45<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_T2_Il2CppString189<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_T2_T3_253<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_T2_f32_125<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_T2_i32_61<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_f32_Il2CppString157<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_f32_T2_221<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_f32_f32_93<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_f32_i32_29<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_i32_Il2CppString141<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_i32_T2_205<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_i32_f32_77<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_T1_i32_i32_13<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: T1,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        T1,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_Il2CppString_Il2CppString165(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_Il2CppString_T1_229<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_Il2CppString_f32_101(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_Il2CppString_i32_37(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_T1_Il2CppString181<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_T1_T2_245<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_T1_f32_117<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_T1_i32_53<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_f32_Il2CppString149(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_f32_T1_213<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_f32_f32_85(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_f32_i32_21(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_i32_Il2CppString133(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_i32_T1_197<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_i32_f32_69(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_f32_i32_i32_5(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: f32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        f32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_Il2CppString_Il2CppString161(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_Il2CppString_T1_225<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_Il2CppString_f32_97(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_Il2CppString_i32_33(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_T1_Il2CppString177<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_T1_T2_241<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_T1_f32_113<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_T1_i32_49<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_f32_Il2CppString145(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_f32_T1_209<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_f32_f32_81(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_f32_i32_17(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_i32_Il2CppString129(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_i32_T1_193<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_i32_f32_65(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_f32_i32_i32_i32_1(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: f32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        f32,
                        i32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_Il2CppString_Il2CppString168(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_Il2CppString_T1_232<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_Il2CppString_f32_104(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_Il2CppString_i32_40(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_T1_Il2CppString184<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_T1_T2_248<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_T1_f32_120<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_T1_i32_56<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_f32_Il2CppString152(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_f32_T1_216<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_f32_f32_88(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_f32_i32_24(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_i32_Il2CppString136(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_i32_T1_200<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_i32_f32_72(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_Il2CppString_i32_i32_8(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_Il2CppString_Il2CppString172<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_Il2CppString_T2_236<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_Il2CppString_f32_108<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_Il2CppString_i32_44<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_T2_Il2CppString188<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: T2,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        T2,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_T2_T3_252<T1, T2, T3>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: T2,
        arg3: T3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        T2,
                        T3,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_T2_f32_124<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: T2,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        T2,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_T2_i32_60<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: T2,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        T2,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_f32_Il2CppString156<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_f32_T2_220<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: f32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        f32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_f32_f32_92<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_f32_i32_28<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_i32_Il2CppString140<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_i32_T2_204<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: i32,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        i32,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_i32_f32_76<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_T1_i32_i32_12<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: T1,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        T1,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_Il2CppString_Il2CppString164(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_Il2CppString_T1_228<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_Il2CppString_f32_100(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_Il2CppString_i32_36(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_T1_Il2CppString180<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_T1_T2_244<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_T1_f32_116<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_T1_i32_52<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_f32_Il2CppString148(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_f32_T1_212<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_f32_f32_84(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_f32_i32_20(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_i32_Il2CppString132(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_i32_T1_196<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_i32_f32_68(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_f32_i32_i32_4(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: f32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        f32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_Il2CppString_Il2CppString160(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_Il2CppString_T1_224<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_Il2CppString_f32_96(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_Il2CppString_i32_32(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_T1_Il2CppString176<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: T1,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        T1,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_T1_T2_240<T1, T2>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: T1,
        arg3: T2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        T1,
                        T2,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_T1_f32_112<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: T1,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        T1,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_T1_i32_48<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: T1,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        T1,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_f32_Il2CppString144(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: f32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_f32_T1_208<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: f32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        f32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_f32_f32_80(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: f32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        f32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_f32_i32_16(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: f32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        f32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_i32_Il2CppString128(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_i32_T1_192<T1>(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: T1,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        i32,
                        T1,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_i32_f32_64(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        i32,
                        f32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_FixedString512Bytes_i32_i32_i32_i32_0(
        formatString: crate::Unity::Collections::FixedString512Bytes,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), crate::Unity::Collections::FixedString512Bytes, 5usize>(
                        "Format"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Format",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes = unsafe {
            cordl_method_info.invoke_unchecked((), (formatString, arg0, arg1, arg2, arg3))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+FixedString")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Collections::FixedString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
