#[cfg(feature = "cordl_class_System+DateTimeParse")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+DateTimeParse")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeParse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeParse";
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
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::Deref for crate::System::DateTimeParse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::DerefMut for crate::System::DateTimeParse {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl crate::System::DateTimeParse {
    #[cfg(feature = "System+DateTimeParse+DS")]
    pub type DS = crate::System::DateTimeParse_DS;
    #[cfg(feature = "System+DateTimeParse+DTT")]
    pub type DTT = crate::System::DateTimeParse_DTT;
    #[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
    pub type MatchNumberDelegate = crate::System::DateTimeParse_MatchNumberDelegate;
    #[cfg(feature = "System+DateTimeParse+TM")]
    pub type TM = crate::System::DateTimeParse_TM;
    pub fn AdjustHour(
        hour: quest_hook::libil2cpp::ByRefMut<i32>,
        timeMark: crate::System::DateTimeParse_TM,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        crate::System::DateTimeParse_TM,
                    ), bool, 2usize>("AdjustHour")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustHour",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (hour, timeMark))? };
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeMark(
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>("AdjustTimeMark")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustTimeMark",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (dtfi, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeZoneToLocal(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        bTimeOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        bool,
                    ), bool, 2usize>("AdjustTimeZoneToLocal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustTimeZoneToLocal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, bTimeOnly))? };
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeZoneToUniversal(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>),
                        bool,
                        1usize,
                    >("AdjustTimeZoneToUniversal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AdjustTimeZoneToUniversal", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckDefaultDateTime(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        cal: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
                        >,
                        crate::System::Globalization::DateTimeStyles,
                    ), bool, 3usize>("CheckDefaultDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckDefaultDateTime",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, cal, styles))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNewValue(
        currentValue: quest_hook::libil2cpp::ByRefMut<i32>,
        newValue: i32,
        patternChar: char,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        i32,
                        char,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 4usize>("CheckNewValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckNewValue",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (currentValue, newValue, patternChar, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeOffsetTimeZonePostProcessing(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        crate::System::Globalization::DateTimeStyles,
                    ), bool, 3usize>("DateTimeOffsetTimeZonePostProcessing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DateTimeOffsetTimeZonePostProcessing",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, result, styles))? };
        Ok(__cordl_ret.into())
    }
    pub fn DetermineTimeZoneAdjustments(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: crate::System::Globalization::DateTimeStyles,
        bTimeOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        crate::System::Globalization::DateTimeStyles,
                        bool,
                    ), bool, 4usize>("DetermineTimeZoneAdjustments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DetermineTimeZoneAdjustments",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, result, styles, bTimeOnly))? };
        Ok(__cordl_ret.into())
    }
    pub fn DoStrictParse(
        s: crate::System::ReadOnlySpan_1<char>,
        formatParam: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::DateTimeStyles,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 5usize>("DoStrictParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoStrictParse",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (s, formatParam, styles, dtfi, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandPredefinedFormat(
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        >,
        parseInfo: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, 4usize>(
                        "ExpandPredefinedFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExpandPredefinedFormat",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (format, dtfi, parseInfo, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfDSN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetDateOfDSN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDateOfDSN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfNDS(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetDateOfNDS")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDateOfNDS",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfNNDS(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetDateOfNNDS")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDateOfNNDS",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateTimeNow(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                    ), crate::System::DateTime, 2usize>("GetDateTimeNow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDateTimeNow",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (result, styles))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateTimeParseException(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Exception>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        1usize,
                    >("GetDateTimeParseException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDateTimeParseException", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> =
            unsafe { cordl_method_info.invoke_unchecked((), (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 4usize>("GetDayOfMN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfMN",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, styles, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetDayOfMNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfMNN",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 4usize>("GetDayOfNM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfNM",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, styles, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 4usize>("GetDayOfNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfNN",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, styles, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetDayOfNNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfNNN",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNNY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetDayOfNNY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfNNY",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetDayOfYM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfYM",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYMN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetDayOfYMN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfYMN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetDayOfYN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfYN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetDayOfYNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDayOfYNN",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultYear(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("GetDefaultYear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDefaultYear",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (result, styles))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHebrewDayOfNM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 3usize>("GetHebrewDayOfNM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHebrewDayOfNM",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetJapaneseCalendarDefaultInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::Calendar,
                        >,
                        0usize,
                    >("GetJapaneseCalendarDefaultInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetJapaneseCalendarDefaultInstance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthDayOrder(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("GetMonthDayOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMonthDayOrder",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (pattern, dtfi, order))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTaiwanCalendarDefaultInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::Calendar,
                        >,
                        0usize,
                    >("GetTaiwanCalendarDefaultInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTaiwanCalendarDefaultInstance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetTimeOfN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTimeOfN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetTimeOfNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTimeOfNN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfNNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                    ), bool, 2usize>("GetTimeOfNNN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTimeOfNNN",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (result, raw))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeZoneName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>),
                        bool,
                        1usize,
                    >("GetTimeZoneName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTimeZoneName", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetYearMonthDayOrder(
        datePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("GetYearMonthDayOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetYearMonthDayOrder",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (datePattern, dtfi, order))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetYearMonthOrder(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("GetYearMonthOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetYearMonthOrder",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (pattern, dtfi, order))? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleTimeZone(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 2usize>("HandleTimeZone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HandleTimeZone",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), bool, 1usize>("IsDigit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsDigit",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn Lex(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtok: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        dtfi: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::DateTimeParse_DS,
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        >,
                        crate::System::Globalization::DateTimeStyles,
                    ), bool, 7usize>("Lex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Lex",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (dps, str, dtok, raw, result, dtfi, styles))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedDayName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchAbbreviatedDayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchAbbreviatedDayName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedMonthName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchAbbreviatedMonthName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchAbbreviatedMonthName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedTimeMark(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
                    ), bool, 3usize>("MatchAbbreviatedTimeMark")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchAbbreviatedTimeMark",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchDayName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchDayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchDayName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchEraName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchEraName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchEraName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchHebrewDigits(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        number: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchHebrewDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchHebrewDigits",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, digitLen, number))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchMonthName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("MatchMonthName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchMonthName",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchTimeMark(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
                    ), bool, 3usize>("MatchTimeMark")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchTimeMark",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchWord(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("MatchWord")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MatchWord",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, target))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseByFormat(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        format: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        parseInfo: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 5usize>("ParseByFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseByFormat",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (str, format, parseInfo, dtfi, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDigits_ByRefMut0(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("ParseDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseDigits",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, digitLen, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDigits_i32_ByRefMut1(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        minDigitLen: i32,
        maxDigitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 4usize>("ParseDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseDigits",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (str, minDigitLen, maxDigitLen, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactMultiple(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                    ), crate::System::DateTime, 4usize>("ParseExactMultiple")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseExactMultiple",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (s, formats, dtfi, style))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), crate::System::DateTime, 5usize>("ParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseExact",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (s, format, dtfi, style, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles0(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                    ), crate::System::DateTime, 4usize>("ParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseExact",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (s, format, dtfi, style))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseFraction(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<f64>,
                    ), bool, 2usize>("ParseFraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseFraction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseFractionExact(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        maxDigitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<f64>,
                    ), bool, 3usize>("ParseFractionExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseFractionExact",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, maxDigitLen, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseISO8601(
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 4usize>("ParseISO8601")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseISO8601",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (raw, str, styles, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseJapaneseEraStart(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 2usize>("ParseJapaneseEraStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseJapaneseEraStart",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSign(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<bool>,
                    ), bool, 2usize>("ParseSign")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseSign",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeZone(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), bool, 2usize>("ParseTimeZone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseTimeZone",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeZoneOffset(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        len: i32,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), bool, 3usize>("ParseTimeZoneOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseTimeZoneOffset",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (str, len, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        styles: crate::System::Globalization::DateTimeStyles,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), crate::System::DateTime, 4usize>("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Parse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (s, dtfi, styles, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles0(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                    ), crate::System::DateTime, 3usize>("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Parse",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime =
            unsafe { cordl_method_info.invoke_unchecked((), (s, dtfi, styles))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDateTimeSuffix(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtok: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
                    ), bool, 3usize>("ProcessDateTimeSuffix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessDateTimeSuffix",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, raw, dtok))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessHebrewTerminalState(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::DateTimeParse_DS,
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 6usize>("ProcessHebrewTerminalState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessHebrewTerminalState",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (dps, str, result, styles, raw, dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminalState(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<crate::System::Globalization::DateTimeStyles>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::DateTimeParse_DS,
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::DateTimeStyles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                    ), bool, 6usize>("ProcessTerminalState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessTerminalState",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (dps, str, result, styles, raw, dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDateDMY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        day: i32,
        month: i32,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        i32,
                        i32,
                        i32,
                    ), bool, 4usize>("SetDateDMY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDateDMY",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, day, month, year))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetDateMDY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        month: i32,
        day: i32,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        i32,
                        i32,
                        i32,
                    ), bool, 4usize>("SetDateMDY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDateMDY",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, month, day, year))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetDateYDM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        day: i32,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        i32,
                        i32,
                        i32,
                    ), bool, 4usize>("SetDateYDM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDateYDM",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, year, day, month))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetDateYMD(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        i32,
                        i32,
                        i32,
                    ), bool, 4usize>("SetDateYMD")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDateYMD",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, year, month, day))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryAdjustYear(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        adjustedYear: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("TryAdjustYear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAdjustYear",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (result, year, adjustedYear))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExactMultiple_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), bool, 6usize>("TryParseExactMultiple")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseExactMultiple",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (s, formats, dtfi, style, result, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExactMultiple_ReadOnlySpan_1_Il2CppArray_DateTimeFormatInfo_DateTimeStyles_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 5usize>("TryParseExactMultiple")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseExactMultiple",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, formats, dtfi, style, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), bool, 6usize>("TryParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseExact",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (s, format, dtfi, style, result, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                    ), bool, 5usize>("TryParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseExact",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, format, dtfi, style, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut2(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 5usize>("TryParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseExact",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, format, dtfi, style, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseQuoteString(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        returnValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 4usize>("TryParseQuoteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParseQuoteString",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (format, pos, result, returnValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                    ), bool, 5usize>("TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParse",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, dtfi, styles, result, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                    ), bool, 4usize>("TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, dtfi, styles, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut2(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::ReadOnlySpan_1<char>,
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
                        crate::System::Globalization::DateTimeStyles,
                        quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
                    ), bool, 4usize>("TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryParse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (s, dtfi, styles, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyValidPunctuation(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>),
                        bool,
                        1usize,
                    >("VerifyValidPunctuation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VerifyValidPunctuation", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (str))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeParse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DateTimeParse_DS {
    #[default]
    BEGIN = 0i32,
    DX_DS = 26i32,
    DX_DSN = 27i32,
    DX_MN = 23i32,
    DX_MNN = 25i32,
    DX_NDS = 28i32,
    DX_NM = 24i32,
    DX_NN = 21i32,
    DX_NNDS = 29i32,
    DX_NNN = 22i32,
    DX_NNY = 38i32,
    DX_YM = 33i32,
    DX_YMN = 31i32,
    DX_YN = 32i32,
    DX_YNN = 30i32,
    D_M = 6i32,
    D_MN = 7i32,
    D_MNd = 9i32,
    D_NDS = 10i32,
    D_NM = 8i32,
    D_NN = 4i32,
    D_NNd = 5i32,
    D_Nd = 3i32,
    D_S = 16i32,
    D_Y = 11i32,
    D_YM = 14i32,
    D_YMd = 15i32,
    D_YN = 12i32,
    D_YNd = 13i32,
    ERROR = 20i32,
    N = 1i32,
    NN = 2i32,
    TX_N = 34i32,
    TX_NN = 35i32,
    TX_NNN = 36i32,
    TX_TS = 37i32,
    T_NNt = 19i32,
    T_Nt = 18i32,
    T_S = 17i32,
}
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeParse_DS {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeParse/DS";
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::DateTimeParse_DS {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::DateTimeParse_DS {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::DateTimeParse_DS {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DS")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::DateTimeParse_DS {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DateTimeParse_DTT {
    #[default]
    DayOfWeek = 11i32,
    End = 0i32,
    Era = 16i32,
    Max = 20i32,
    MonthDatesep = 8i32,
    MonthEnd = 6i32,
    MonthSpace = 7i32,
    NumAmpm = 2i32,
    NumDatesep = 4i32,
    NumDatesuff = 9i32,
    NumEnd = 1i32,
    NumLocalTimeMark = 19i32,
    NumSpace = 3i32,
    NumTimesep = 5i32,
    NumTimesuff = 10i32,
    NumUTCTimeMark = 17i32,
    TimeZone = 15i32,
    Unk = 18i32,
    YearDateSep = 13i32,
    YearEnd = 14i32,
    YearSpace = 12i32,
}
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeParse_DTT {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeParse/DTT";
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::DateTimeParse_DTT {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::DateTimeParse_DTT {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::DateTimeParse_DTT {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+DTT")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::DateTimeParse_DTT {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+MatchNumberDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse_MatchNumberDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_System+DateTimeParse+MatchNumberDelegate")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeParse_MatchNumberDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeParse/MatchNumberDelegate";
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
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::Deref for crate::System::DateTimeParse_MatchNumberDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::DerefMut for crate::System::DateTimeParse_MatchNumberDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl crate::System::DateTimeParse_MatchNumberDelegate {
    pub fn Invoke(
        &mut self,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Invoke",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (str, digitLen, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (object, method))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+MatchNumberDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeParse_MatchNumberDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DateTimeParse_TM {
    #[default]
    AM = 0i32,
    NotSet = -1i32,
    PM = 1i32,
}
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeParse_TM {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeParse/TM";
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
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::DateTimeParse_TM {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::DateTimeParse_TM {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::DateTimeParse_TM {
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
#[cfg(feature = "cordl_class_System+DateTimeParse+TM")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::DateTimeParse_TM {
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
