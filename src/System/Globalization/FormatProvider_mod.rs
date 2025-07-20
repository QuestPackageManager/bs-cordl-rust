#[cfg(feature = "System+Globalization+FormatProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+FormatProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::FormatProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "FormatProvider";
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
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl crate::System::Globalization::FormatProvider {
    #[cfg(feature = "System+Globalization+FormatProvider+Number")]
    pub type Number = crate::System::Globalization::FormatProvider_Number;
    pub fn FormatBigInteger(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        precision: i32,
        scale: i32,
        sign: bool,
        format: crate::System::ReadOnlySpan_1<char>,
        numberFormatInfo: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        digits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            i32,
                            i32,
                            bool,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("FormatBigInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatBigInteger", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sb,
                        precision,
                        scale,
                        sign,
                        format,
                        numberFormatInfo,
                        digits,
                        startIndex,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToBigInteger(
        s: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        numberFormatInfo: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        receiver: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        precision: quest_hook::libil2cpp::ByRefMut<i32>,
        scale: quest_hook::libil2cpp::ByRefMut<i32>,
        sign: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::Globalization::NumberStyles,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        bool,
                        7usize,
                    >("TryStringToBigInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryStringToBigInteger", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (s, styles, numberFormatInfo, receiver, precision, scale, sign),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::FormatProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider_Number {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::FormatProvider_Number {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "FormatProvider/Number";
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
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider_Number {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider_Number {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl crate::System::Globalization::FormatProvider_Number {
    #[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Globalization::Number_FormatProvider_NumberBuffer;
    pub fn FindSection(
        format: crate::System::ReadOnlySpan_1<char>,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32),
                        i32,
                        2usize,
                    >("FindSection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindSection", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (format, section))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatCurrency(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("FormatCurrency")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatCurrency", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatExponent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        value: i32,
        expChar: char,
        minDigits: i32,
        positiveSign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            i32,
                            char,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("FormatExponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatExponent", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (sb, info, value, expChar, minDigits, positiveSign),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatFixed(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        groupDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        sDecimal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("FormatFixed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatFixed", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sb,
                        number,
                        nMinDigits,
                        nMaxDigits,
                        info,
                        groupDigits,
                        sDecimal,
                        sGroup,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatGeneral(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
        bSuppressScientific: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            char,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("FormatGeneral")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatGeneral", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sb,
                        number,
                        nMinDigits,
                        nMaxDigits,
                        info,
                        expChar,
                        bSuppressScientific,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatNumber(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("FormatNumber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatNumber", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatPercent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("FormatPercent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatPercent", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatScientific(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            char,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("FormatScientific")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FormatScientific", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (sb, number, nMinDigits, nMaxDigits, info, expChar),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToDecChars(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            u32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Int32ToDecChars")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Int32ToDecChars", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (buffer, index, value, digits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsWhite(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(char), bool, 1usize>("IsWhite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsWhite", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars_Il2CppObject1(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("MatchChars")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchChars", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (p, pEnd, str))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars_Il2CppString0(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("MatchChars")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchChars", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (p, pEnd, str))? };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToString(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        format: char,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        isDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            char,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("NumberToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NumberToString", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (sb, number, format, nMaxDigits, info, isDecimal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToStringFormat(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Text::ValueStringBuilder,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("NumberToStringFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NumberToStringFormat", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, format, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseFormatSpecifier(
        format: crate::System::ReadOnlySpan_1<char>,
        digits: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        char,
                        2usize,
                    >("ParseFormatSpecifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseFormatSpecifier", 2usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (format, digits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumber(
        str: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        strEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        options: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        numfmt: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Globalization::NumberStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            bool,
                        ),
                        bool,
                        7usize,
                    >("ParseNumber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseNumber", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (str, strEnd, options, number, sb, numfmt, parseDecimal),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RoundNumber(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RoundNumber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RoundNumber", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (number, pos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrailingZeros(
        s: crate::System::ReadOnlySpan_1<char>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32),
                        bool,
                        2usize,
                    >("TrailingZeros")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TrailingZeros", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, index))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToNumber(
        str: crate::System::ReadOnlySpan_1<char>,
        options: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        numfmt: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::Globalization::NumberStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::Number_FormatProvider_NumberBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::NumberFormatInfo,
                            >,
                            bool,
                        ),
                        bool,
                        6usize,
                    >("TryStringToNumber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryStringToNumber", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked((), (str, options, number, sb, numfmt, parseDecimal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn wcslen(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        i32,
                        1usize,
                    >("wcslen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "wcslen", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::FormatProvider_Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Number_FormatProvider_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub sign: bool,
    pub overrideDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "FormatProvider/Number/NumberBuffer";
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
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
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
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
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
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
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
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
impl crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    pub fn get_digits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_digits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_digits", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
