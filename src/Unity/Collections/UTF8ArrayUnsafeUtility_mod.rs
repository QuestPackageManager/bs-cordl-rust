#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8ArrayUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::UTF8ArrayUnsafeUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "UTF8ArrayUnsafeUtility";
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
#[cfg(feature = "Unity+Collections+UTF8ArrayUnsafeUtility")]
impl std::ops::Deref for crate::Unity::Collections::UTF8ArrayUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+UTF8ArrayUnsafeUtility")]
impl std::ops::DerefMut for crate::Unity::Collections::UTF8ArrayUnsafeUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+UTF8ArrayUnsafeUtility")]
impl crate::Unity::Collections::UTF8ArrayUnsafeUtility {
    #[cfg(feature = "Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
    pub type Comparison = crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison;
    pub fn AppendUTF8Bytes(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<i32>,
        destCapacity: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FormatError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::FormatError,
                        5usize,
                    >("AppendUTF8Bytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendUTF8Bytes", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FormatError = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dest, destLength, destCapacity, src, srcLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_i32_1(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUTF8MaxLengthInBytes: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_u16_0(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUTF8MaxLengthInBytes: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u16,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Append_u16_2(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUCS2MaxLengthInChars: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u16,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Append",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUCS2MaxLengthInChars, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_i32_0(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<i32>,
        destUTF8MaxLengthInBytes: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_i32_2(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<i32>,
        destUTF8MaxLengthInBytes: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_i32_4(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<i32>,
        destUCS2MaxLengthInChars: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUCS2MaxLengthInChars, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_u16_i32_1(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUTF8MaxLengthInBytes: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_u16_u16_3(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUTF8MaxLengthInBytes: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u16,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUTF8MaxLengthInBytes, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_u16_u16_5(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: quest_hook::libil2cpp::ByRefMut<u16>,
        destUCS2MaxLengthInChars: u16,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::CopyError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                            u16,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            u16,
                        ),
                        crate::Unity::Collections::CopyError,
                        5usize,
                    >("Copy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Copy",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::CopyError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dest, destLength, destUCS2MaxLengthInChars, src, srcLength),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EqualsUTF8Bytes(
        aBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        aLength: i32,
        bBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bLength: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        bool,
                        4usize,
                    >("EqualsUTF8Bytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EqualsUTF8Bytes", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (aBytes, aLength, bBytes, bLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StrCmp_Il2CppObject_i32_Il2CppObject_i32_0(
        utf8BufferA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8LengthInBytesA: i32,
        utf8BufferB: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8LengthInBytesB: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("StrCmp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "StrCmp",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (utf8BufferA, utf8LengthInBytesA, utf8BufferB, utf8LengthInBytesB),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StrCmp_Il2CppObject_i32_Il2CppObject_i32_1(
        utf8BufferA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8LengthInBytesA: i32,
        runeBufferB: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        lengthInRunesB: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("StrCmp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "StrCmp",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (utf8BufferA, utf8LengthInBytesA, runeBufferB, lengthInRunesB),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StrCmp_Il2CppObject_i32_Il2CppObject_i32_2(
        utf16BufferA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf16LengthInCharsA: i32,
        utf16BufferB: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf16LengthInCharsB: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("StrCmp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "StrCmp",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        utf16BufferA,
                        utf16LengthInCharsA,
                        utf16BufferB,
                        utf16LengthInCharsB,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StrCmp_Il2CppObject_i32_Il2CppObject_i32_3(
        utf8Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8LengthInBytes: i32,
        utf16Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf16LengthInChars: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("StrCmp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "StrCmp",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (utf8Buffer, utf8LengthInBytes, utf16Buffer, utf16LengthInChars),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StrCmp_Il2CppObject_i32_Il2CppObject_i32_4(
        utf16Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf16LengthInChars: i32,
        utf8Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        utf8LengthInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("StrCmp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "StrCmp",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (utf16Buffer, utf16LengthInChars, utf8Buffer, utf8LengthInBytes),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Collections::UTF8ArrayUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UTF8ArrayUnsafeUtility_Comparison {
    pub terminates: bool,
    pub result: i32,
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "UTF8ArrayUnsafeUtility/Comparison";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
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
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
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
#[cfg(feature = "cordl_class_Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+UTF8ArrayUnsafeUtility+Comparison")]
impl crate::Unity::Collections::UTF8ArrayUnsafeUtility_Comparison {
    pub fn _ctor(
        &mut self,
        runeA: crate::Unity::Collections::Unicode_Rune,
        errorA: crate::Unity::Collections::ConversionError,
        runeB: crate::Unity::Collections::Unicode_Rune,
        errorB: crate::Unity::Collections::ConversionError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::Unicode_Rune,
                            crate::Unity::Collections::ConversionError,
                            crate::Unity::Collections::Unicode_Rune,
                            crate::Unity::Collections::ConversionError,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (runeA, errorA, runeB, errorB))?
        };
        Ok(__cordl_ret.into())
    }
}
