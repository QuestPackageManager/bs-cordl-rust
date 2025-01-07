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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (initialValue, culture, targetType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertOrCast(
        initialValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertOrCast", (initialValue, culture, targetType))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCastConverter", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecimalTryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecimalTryParse", (chars, start, length, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTypeAssignable(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        initialType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureTypeAssignable", (value, initialType, targetType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        i: crate::System::Numerics::BigInteger,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBigInteger", (i, targetType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode_ByRefMut1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isEnum: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeCode", (t, isEnum))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode_Type0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeCode", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeInformation(
        convertable: quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::TypeInformation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::TypeInformation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeInformation", (convertable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32TryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int32TryParse", (chars, start, length, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int64TryParse(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        length: i32,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Utilities::ParseResult> {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ParseResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int64TryParse", (chars, start, length, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsConvertible(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConvertible", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInteger(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInteger", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeSpan(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTimeSpan", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBigInteger(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBigInteger", (value))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvert", (initialValue, culture, targetType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertGuid(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        g: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertGuid", (s, g))?;
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
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertInternal", (initialValue, culture, targetType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryHexTextToInt(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        end: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryHexTextToInt", (text, start, end, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn VersionTryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Version>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VersionTryParse", (input, result))?;
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
    const CLASS_NAME: &'static str = "ConvertResult";
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
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
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
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
