#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct StringUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::StringUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "StringUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::StringUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::StringUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl crate::Newtonsoft::Json::Utilities::StringUtils {
    pub const CarriageReturn: char = '\r';
    pub const CarriageReturnLineFeed: &'static str = "\\r\\n";
    pub const Empty: &'static str = "";
    pub const LineFeed: char = '\n';
    pub const Tab: char = '\t';
    #[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
    pub type SeparatedCaseState = crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState;
    pub fn CreateStringWriter(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StringWriter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StringWriter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStringWriter", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWith", (source, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForgivingCaseSensitiveFind<TSource>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        valueSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        testValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<TSource>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TSource = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ForgivingCaseSensitiveFind", (source, valueSelector, testValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatWith_Il2CppArray4(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatWith", (format, provider, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatWith_Il2CppObject0(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatWith", (format, provider, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatWith_Il2CppObject_Il2CppObject1(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatWith", (format, provider, arg0, arg1))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatWith_Il2CppObject_Il2CppObject_Il2CppObject2(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatWith", (format, provider, arg0, arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatWith_Il2CppObject_Il2CppObject_Il2CppObject_Il2CppObject3(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        arg3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatWith", (format, provider, arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        c: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHighSurrogate(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHighSurrogate", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLowSurrogate(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLowSurrogate", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullOrEmpty(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullOrEmpty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWhiteSpace", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Replace", (s, oldValue, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartsWith", (source, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCamelCase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToCamelCase", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCharAsUnicode(
        c: char,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCharAsUnicode", (c, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToKebabCase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToKebabCase", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLower(c: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLower", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSeparatedCase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        separator: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSeparatedCase", (s, separator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSnakeCase(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToSnakeCase", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Trim", (s, start, length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::StringUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringUtils_SeparatedCaseState {
    #[default]
    Lower = 1i32,
    NewWord = 3i32,
    Start = 0i32,
    Upper = 2i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "StringUtils/SeparatedCaseState";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState {
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
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState {
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
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState {
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
