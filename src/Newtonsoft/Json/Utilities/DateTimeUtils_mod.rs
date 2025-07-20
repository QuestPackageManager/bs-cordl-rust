#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "DateTimeUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    pub const DaysPer100Years: i32 = 36524i32;
    pub const DaysPer400Years: i32 = 146097i32;
    pub const DaysPer4Years: i32 = 1461i32;
    pub const DaysPerYear: i32 = 365i32;
    pub const IsoDateFormat: &'static str = "yyyy-MM-ddTHH:mm:ss.FFFFFFFK";
    pub const TicksPerDay: i64 = 864000000000i64;
    pub fn ConvertDateTimeToJavaScriptTicks_DateTime1(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime),
                        i64,
                        1usize,
                    >("ConvertDateTimeToJavaScriptTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertDateTimeToJavaScriptTicks", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (dateTime))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDateTimeToJavaScriptTicks_TimeSpan0(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime, crate::System::TimeSpan),
                        i64,
                        2usize,
                    >("ConvertDateTimeToJavaScriptTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertDateTimeToJavaScriptTicks", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (dateTime, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDateTimeToJavaScriptTicks__cordl_bool2(
        dateTime: crate::System::DateTime,
        convertToUtc: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime, bool),
                        i64,
                        2usize,
                    >("ConvertDateTimeToJavaScriptTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertDateTimeToJavaScriptTicks", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (dateTime, convertToUtc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertJavaScriptTicksToDateTime(
        javaScriptTicks: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i64),
                        crate::System::DateTime,
                        1usize,
                    >("ConvertJavaScriptTicksToDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertJavaScriptTicksToDateTime", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (javaScriptTicks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyIntToCharArray(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: i32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("CopyIntToCharArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CopyIntToCharArray", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (chars, start, value, digits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDateTime(
        dateTimeParser: crate::Newtonsoft::Json::Utilities::DateTimeParser,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Newtonsoft::Json::Utilities::DateTimeParser),
                        crate::System::DateTime,
                        1usize,
                    >("CreateDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateDateTime", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (dateTimeParser))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDateTime(
        value: crate::System::DateTime,
        timeZone: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                        ),
                        crate::System::DateTime,
                        2usize,
                    >("EnsureDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnsureDateTime", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value, timeZone))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDateValues(
        td: crate::System::DateTime,
        year: quest_hook::libil2cpp::ByRefMut<i32>,
        month: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetDateValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDateValues", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (td, year, month, day))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset(
        d: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime),
                        crate::System::TimeSpan,
                        1usize,
                    >("GetUtcOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUtcOffset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked((), (d))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwitchToLocalTime(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime),
                        crate::System::DateTime,
                        1usize,
                    >("SwitchToLocalTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SwitchToLocalTime", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwitchToUtcTime(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime),
                        crate::System::DateTime,
                        1usize,
                    >("SwitchToUtcTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SwitchToUtcTime", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializationMode(
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::XmlDateTimeSerializationMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTimeKind),
                        crate::System::Xml::XmlDateTimeSerializationMode,
                        1usize,
                    >("ToSerializationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToSerializationMode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::XmlDateTimeSerializationMode = unsafe {
            method.invoke_unchecked((), (kind))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTicks_DateTime0(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime),
                        i64,
                        1usize,
                    >("ToUniversalTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToUniversalTicks", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (dateTime))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTicks_TimeSpan1(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::DateTime, crate::System::TimeSpan),
                        i64,
                        2usize,
                    >("ToUniversalTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToUniversalTicks", 2usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (dateTime, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeExact(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        ),
                        bool,
                        5usize,
                    >("TryParseDateTimeExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeExact", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (text, dateTimeZoneHandling, dateFormatString, culture, dt),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeIso(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        ),
                        bool,
                        3usize,
                    >("TryParseDateTimeIso")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeIso", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (text, dateTimeZoneHandling, dt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeMicrosoft(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        ),
                        bool,
                        3usize,
                    >("TryParseDateTimeMicrosoft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeMicrosoft", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (text, dateTimeZoneHandling, dt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetExact(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::DateTimeOffset,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryParseDateTimeOffsetExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeOffsetExact", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (text, dateFormatString, culture, dt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetIso(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::DateTimeOffset,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryParseDateTimeOffsetIso")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeOffsetIso", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (text, dt))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetMicrosoft(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::DateTimeOffset,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryParseDateTimeOffsetMicrosoft")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeOffsetMicrosoft", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (text, dt))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffset_Il2CppString1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::DateTimeOffset,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryParseDateTimeOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeOffset", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, dateFormatString, culture, dt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffset_StringReference0(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::DateTimeOffset,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryParseDateTimeOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTimeOffset", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, dateFormatString, culture, dt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTime_Il2CppString1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        ),
                        bool,
                        5usize,
                    >("TryParseDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTime", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (s, dateTimeZoneHandling, dateFormatString, culture, dt),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTime_StringReference0(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            crate::Newtonsoft::Json::DateTimeZoneHandling,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                        ),
                        bool,
                        5usize,
                    >("TryParseDateTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseDateTime", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (s, dateTimeZoneHandling, dateFormatString, culture, dt),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseMicrosoftDate(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        ticks: quest_hook::libil2cpp::ByRefMut<i64>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
        kind: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeKind>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeKind>,
                        ),
                        bool,
                        4usize,
                    >("TryParseMicrosoftDate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryParseMicrosoftDate", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (text, ticks, offset, kind))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryReadOffset(
        offsetText: crate::Newtonsoft::Json::Utilities::StringReference,
        startIndex: i32,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Newtonsoft::Json::Utilities::StringReference,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                        ),
                        bool,
                        3usize,
                    >("TryReadOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryReadOffset", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (offsetText, startIndex, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UniversalTicksToJavaScriptTicks(
        universalTicks: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i64),
                        i64,
                        1usize,
                    >("UniversalTicksToJavaScriptTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UniversalTicksToJavaScriptTicks", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (universalTicks))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeOffset(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        offset: crate::System::TimeSpan,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            crate::System::TimeSpan,
                            crate::Newtonsoft::Json::DateFormatHandling,
                        ),
                        i32,
                        4usize,
                    >("WriteDateTimeOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDateTimeOffset", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (chars, start, offset, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeOffsetString(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: crate::System::DateTimeOffset,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        formatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            crate::System::DateTimeOffset,
                            crate::Newtonsoft::Json::DateFormatHandling,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("WriteDateTimeOffsetString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDateTimeOffsetString", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (writer, value, format, formatString, culture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeString_Il2CppArray_i32_DateTime_Nullable_1_DateTimeKind_DateFormatHandling1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: crate::System::DateTime,
        offset: crate::System::Nullable_1<crate::System::TimeSpan>,
        kind: crate::System::DateTimeKind,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            crate::System::DateTime,
                            crate::System::Nullable_1<crate::System::TimeSpan>,
                            crate::System::DateTimeKind,
                            crate::Newtonsoft::Json::DateFormatHandling,
                        ),
                        i32,
                        6usize,
                    >("WriteDateTimeString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDateTimeString", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (chars, start, value, offset, kind, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeString_TextWriter_DateTime_DateFormatHandling_Il2CppString_CultureInfo0(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: crate::System::DateTime,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        formatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            crate::System::DateTime,
                            crate::Newtonsoft::Json::DateFormatHandling,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("WriteDateTimeString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDateTimeString", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (writer, value, format, formatString, culture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDefaultIsoDate(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            crate::System::DateTime,
                        ),
                        i32,
                        3usize,
                    >("WriteDefaultIsoDate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteDefaultIsoDate", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (chars, start, dt))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
