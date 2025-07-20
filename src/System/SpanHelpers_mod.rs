#[cfg(feature = "System+SpanHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct SpanHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+SpanHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::SpanHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "SpanHelpers";
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
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::Deref for crate::System::SpanHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::DerefMut for crate::System::SpanHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl crate::System::SpanHelpers {
    pub fn ClearWithReferences(
        ip: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pointerSizeLength: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearWithReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ClearWithReferences", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ip, pointerSizeLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearWithoutReferences(
        b: quest_hook::libil2cpp::ByRefMut<u8>,
        byteLength: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<u8>, u64),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearWithoutReferences")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ClearWithoutReferences", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (b, byteLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithCultureHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        compareInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CompareInfo,
                            >,
                        ),
                        bool,
                        3usize,
                    >("EndsWithCultureHelper")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EndsWithCultureHelper", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (span, value, compareInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithCultureIgnoreCaseHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        compareInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CompareInfo,
                            >,
                        ),
                        bool,
                        3usize,
                    >("EndsWithCultureIgnoreCaseHelper")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EndsWithCultureIgnoreCaseHelper", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (span, value, compareInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithOrdinalIgnoreCaseHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                        ),
                        bool,
                        2usize,
                    >("EndsWithOrdinalIgnoreCaseHelper")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EndsWithOrdinalIgnoreCaseHelper", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (span, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny_ByRefMut_i32_ByRefMut_i32_0(
        searchSpace: quest_hook::libil2cpp::ByRefMut<u8>,
        searchSpaceLength: i32,
        value: quest_hook::libil2cpp::ByRefMut<u8>,
        valueLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<u8>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<u8>,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfAny")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOfAny", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (searchSpace, searchSpaceLength, value, valueLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny_ByRefMut_i32_ByRefMut_i32_1<T>(
        searchSpace: quest_hook::libil2cpp::ByRefMut<T>,
        searchSpaceLength: i32,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        valueLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<T>,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfAny")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOfAny", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (searchSpace, searchSpaceLength, value, valueLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_T2<T>(
        searchSpace: quest_hook::libil2cpp::ByRefMut<T>,
        value: T,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<T>, T, i32),
                        i32,
                        3usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOf", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (searchSpace, value, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf__cordl_char1(
        searchSpace: quest_hook::libil2cpp::ByRefMut<char>,
        value: char,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<char>, char, i32),
                        i32,
                        3usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOf", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (searchSpace, value, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_u8_0(
        searchSpace: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<u8>, u8, i32),
                        i32,
                        3usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOf", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (searchSpace, value, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf(
        searchSpace: quest_hook::libil2cpp::ByRefMut<char>,
        value: char,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<char>, char, i32),
                        i32,
                        3usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LastIndexOf", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (searchSpace, value, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LocateFirstFoundChar_Vector_1_0(
        _cordl_match: crate::System::Numerics::Vector_1<u16>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Numerics::Vector_1<u16>),
                        i32,
                        1usize,
                    >("LocateFirstFoundChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LocateFirstFoundChar", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (_cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn LocateFirstFoundChar_u64_1(
        _cordl_match: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u64), i32, 1usize>("LocateFirstFoundChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LocateFirstFoundChar", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (_cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn LocateLastFoundChar_Vector_1_0(
        _cordl_match: crate::System::Numerics::Vector_1<u16>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Numerics::Vector_1<u16>),
                        i32,
                        1usize,
                    >("LocateLastFoundChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LocateLastFoundChar", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (_cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn LocateLastFoundChar_u64_1(
        _cordl_match: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u64), i32, 1usize>("LocateLastFoundChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LocateLastFoundChar", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (_cordl_match))? };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceCompareTo(
        first: quest_hook::libil2cpp::ByRefMut<char>,
        firstLength: i32,
        second: quest_hook::libil2cpp::ByRefMut<char>,
        secondLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<char>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<char>,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("SequenceCompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SequenceCompareTo", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (first, firstLength, second, secondLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_i32_1<T>(
        first: quest_hook::libil2cpp::ByRefMut<T>,
        second: quest_hook::libil2cpp::ByRefMut<T>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<T>,
                            quest_hook::libil2cpp::ByRefMut<T>,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("SequenceEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SequenceEqual", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (first, second, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_u64_0(
        first: quest_hook::libil2cpp::ByRefMut<u8>,
        second: quest_hook::libil2cpp::ByRefMut<u8>,
        length: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<u8>,
                            quest_hook::libil2cpp::ByRefMut<u8>,
                            u64,
                        ),
                        bool,
                        3usize,
                    >("SequenceEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SequenceEqual", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (first, second, length))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::SpanHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
