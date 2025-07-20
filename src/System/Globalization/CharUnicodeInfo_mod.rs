#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CharUnicodeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::CharUnicodeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "CharUnicodeInfo";
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
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl std::ops::Deref for crate::System::Globalization::CharUnicodeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl std::ops::DerefMut for crate::System::Globalization::CharUnicodeInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl crate::System::Globalization::CharUnicodeInfo {
    pub fn GetNumericValue(ch: char) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), f64, 1usize>("GetNumericValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNumericValue", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory_Il2CppString_i32_1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        crate::System::Globalization::UnicodeCategory,
                        2usize,
                    >("GetUnicodeCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnicodeCategory", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = unsafe {
            method.invoke_unchecked((), (s, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory__cordl_char0(
        ch: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (char),
                        crate::System::Globalization::UnicodeCategory,
                        1usize,
                    >("GetUnicodeCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnicodeCategory", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = unsafe {
            method.invoke_unchecked((), (ch))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory_i32_2(
        codePoint: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::System::Globalization::UnicodeCategory,
                        1usize,
                    >("GetUnicodeCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnicodeCategory", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = unsafe {
            method.invoke_unchecked((), (codePoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalConvertToUtf32_ByRefMut1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        charLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        i32,
                        3usize,
                    >("InternalConvertToUtf32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalConvertToUtf32", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (s, index, charLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalConvertToUtf32_Il2CppString_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        i32,
                        2usize,
                    >("InternalConvertToUtf32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalConvertToUtf32", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (s, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetCategoryValue(
        ch: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32),
                        u8,
                        2usize,
                    >("InternalGetCategoryValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalGetCategoryValue", 2usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (ch, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetNumericValue(ch: i32) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), f64, 1usize>("InternalGetNumericValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalGetNumericValue", 1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetUnicodeCategory_ByRefMut1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        charLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        crate::System::Globalization::UnicodeCategory,
                        3usize,
                    >("InternalGetUnicodeCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalGetUnicodeCategory", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = unsafe {
            method.invoke_unchecked((), (str, index, charLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetUnicodeCategory_Il2CppString_i32_0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        crate::System::Globalization::UnicodeCategory,
                        2usize,
                    >("InternalGetUnicodeCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalGetUnicodeCategory", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = unsafe {
            method.invoke_unchecked((), (value, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace_Il2CppString_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        bool,
                        2usize,
                    >("IsWhiteSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsWhiteSpace", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace__cordl_char1(c: char) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), bool, 1usize>("IsWhiteSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsWhiteSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoriesValue() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_CategoriesValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CategoriesValue", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel1Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_CategoryLevel1Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CategoryLevel1Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel2Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_CategoryLevel2Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CategoryLevel2Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel3Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_CategoryLevel3Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CategoryLevel3Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel1Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_NumericLevel1Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NumericLevel1Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel2Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_NumericLevel2Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NumericLevel2Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel3Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_NumericLevel3Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NumericLevel3Index", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericValues() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_NumericValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NumericValues", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::CharUnicodeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
