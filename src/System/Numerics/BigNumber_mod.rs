#[cfg(feature = "System+Numerics+BigNumber")]
#[repr(C)]
#[derive(Debug)]
pub struct BigNumber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+BigNumber")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Numerics::BigNumber {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "BigNumber";
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
#[cfg(feature = "System+Numerics+BigNumber")]
impl std::ops::Deref for crate::System::Numerics::BigNumber {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl std::ops::DerefMut for crate::System::Numerics::BigNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl crate::System::Numerics::BigNumber {
    #[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
    pub type BigNumberBuffer = crate::System::Numerics::BigNumber_BigNumberBuffer;
    pub fn FormatBigIntegerToHex(
        targetSpan: bool,
        value: crate::System::Numerics::BigInteger,
        format: char,
        digits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        spanSuccess: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatBigIntegerToHex",
                (
                    targetSpan,
                    value,
                    format,
                    digits,
                    info,
                    destination,
                    charsWritten,
                    spanSuccess,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatBigInteger_BigInteger_Il2CppString_NumberFormatInfo0(
        value: crate::System::Numerics::BigInteger,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatBigInteger", (value, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatBigInteger__cordl_bool_BigInteger_Il2CppString_ReadOnlySpan_1_NumberFormatInfo_Span_1_ByRefMut_ByRefMut1(
        targetSpan: bool,
        value: crate::System::Numerics::BigInteger,
        formatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatSpan: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        spanSuccess: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatBigInteger",
                (
                    targetSpan,
                    value,
                    formatString,
                    formatSpan,
                    info,
                    destination,
                    charsWritten,
                    spanSuccess,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HexNumberToBigInteger(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Numerics::BigNumber_BigNumberBuffer,
        >,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Numerics::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexNumberToBigInteger", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToBigInteger(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Numerics::BigNumber_BigNumberBuffer,
        >,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Numerics::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToBigInteger", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseBigInteger_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseBigInteger", (value, style, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseBigInteger_ReadOnlySpan_1_1(
        value: crate::System::ReadOnlySpan_1<char>,
        style: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseBigInteger", (value, style, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFormatSpecifier(
        format: crate::System::ReadOnlySpan_1<char>,
        digits: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseFormatSpecifier", (format, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseBigInteger(
        value: crate::System::ReadOnlySpan_1<char>,
        style: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Numerics::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseBigInteger", (value, style, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryValidateParseStyleInteger(
        style: crate::System::Globalization::NumberStyles,
        e: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::ArgumentException>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryValidateParseStyleInteger", (style, e))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+BigNumber")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::BigNumber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BigNumber_BigNumberBuffer {
    pub digits: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub precision: i32,
    pub scale: i32,
    pub sign: bool,
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Numerics::BigNumber_BigNumberBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "BigNumber/BigNumberBuffer";
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
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Numerics::BigNumber_BigNumberBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Numerics::BigNumber_BigNumberBuffer {
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
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Numerics::BigNumber_BigNumberBuffer {
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
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Numerics::BigNumber_BigNumberBuffer {
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
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Numerics::BigNumber_BigNumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+BigNumber+BigNumberBuffer")]
impl crate::System::Numerics::BigNumber_BigNumberBuffer {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::System::Numerics::BigNumber_BigNumberBuffer,
    > {
        let __cordl_ret: crate::System::Numerics::BigNumber_BigNumberBuffer = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
}
