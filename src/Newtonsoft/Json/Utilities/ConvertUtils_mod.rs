#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ConvertUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "ConvertUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl crate::Newtonsoft::Json::Utilities::ConvertUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
    pub type ConvertResult = crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult;
    pub fn Convert(
        initialValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Convert",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (initialValue, culture, targetType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertOrCast(
        initialValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("ConvertOrCast")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertOrCast", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (initialValue, culture, targetType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCastConverter(
        t: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        1usize,
                    >("CreateCastConverter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCastConverter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn DecimalTryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
                        ),
                        crate::Newtonsoft::Json::Utilities::ParseResult,
                        4usize,
                    >("DecimalTryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecimalTryParse", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = unsafe {
            cordl_method_info.invoke_unchecked((), (chars, start, length, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTypeAssignable(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        initialType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("EnsureTypeAssignable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureTypeAssignable", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (value, initialType, targetType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        i: crate::System::Numerics::BigInteger,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Numerics::BigInteger,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("FromBigInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromBigInteger", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked((), (i, targetType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode_ByRefMut1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isEnum: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
                        2usize,
                    >("GetTypeCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeCode", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = unsafe {
            cordl_method_info.invoke_unchecked((), (t, isEnum))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode_Type0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
                        1usize,
                    >("GetTypeCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeCode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = unsafe {
            cordl_method_info.invoke_unchecked((), (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeInformation(
        convertable: quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::TypeInformation>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IConvertible>),
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Utilities::TypeInformation,
                        >,
                        1usize,
                    >("GetTypeInformation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeInformation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::TypeInformation,
        > = unsafe { cordl_method_info.invoke_unchecked((), (convertable))? };
        Ok(__cordl_ret.into())
    }
    pub fn Int32TryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        crate::Newtonsoft::Json::Utilities::ParseResult,
                        4usize,
                    >("Int32TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Int32TryParse", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = unsafe {
            cordl_method_info.invoke_unchecked((), (chars, start, length, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Int64TryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        crate::Newtonsoft::Json::Utilities::ParseResult,
                        4usize,
                    >("Int64TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Int64TryParse", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = unsafe {
            cordl_method_info.invoke_unchecked((), (chars, start, length, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsConvertible(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsConvertible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsConvertible", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInteger(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("IsInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInteger", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeSpan(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::System::TimeSpan,
                        1usize,
                    >("ParseTimeSpan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseTimeSpan", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            cordl_method_info.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBigInteger(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        crate::System::Numerics::BigInteger,
                        1usize,
                    >("ToBigInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToBigInteger", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Numerics::BigInteger = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryConvert(
        initialValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryConvert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryConvert", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (initialValue, culture, targetType, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertGuid(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        g: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
                        ),
                        bool,
                        2usize,
                    >("TryConvertGuid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryConvertGuid", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (s, g))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertInternal(
        initialValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult,
                        4usize,
                    >("TryConvertInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryConvertInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult = unsafe {
            cordl_method_info
                .invoke_unchecked((), (initialValue, culture, targetType, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryHexTextToInt(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        end: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        4usize,
                    >("TryHexTextToInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryHexTextToInt", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (text, start, end, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VersionTryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Version>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::System::Version>,
                            >,
                        ),
                        bool,
                        2usize,
                    >("VersionTryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VersionTryParse", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (input, result))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConvertUtils_ConvertResult {
    #[default]
    CannotConvertNull = 1i32,
    NoValidConversion = 3i32,
    NotInstantiableType = 2i32,
    Success = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "ConvertUtils/ConvertResult";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
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
