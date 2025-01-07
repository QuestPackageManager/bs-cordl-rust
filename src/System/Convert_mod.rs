#[cfg(feature = "System+Convert")]
#[repr(C)]
#[derive(Debug)]
pub struct Convert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Convert")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Convert {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Convert";
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
#[cfg(feature = "System+Convert")]
impl std::ops::Deref for crate::System::Convert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Convert")]
impl std::ops::DerefMut for crate::System::Convert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Convert")]
impl crate::System::Convert {
    pub fn ChangeType_Type1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        conversionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeType", (value, conversionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType_TypeCode_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        typeCode: crate::System::TypeCode,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeType", (value, typeCode, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType_Type_IFormatProvider2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        conversionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeType", (value, conversionType, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToBase64Array(
        outChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        offset: i32,
        length: i32,
        insertLineBreaks: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertToBase64Array",
                (outChars, inData, offset, length, insertLineBreaks),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyToTempBufferWithoutWhiteSpace(
        chars: crate::System::ReadOnlySpan_1<char>,
        tempBuffer: crate::System::Span_1<char>,
        consumed: quest_hook::libil2cpp::ByRefMut<i32>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyToTempBufferWithoutWhiteSpace",
                (chars, tempBuffer, consumed, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        encodedChars: quest_hook::libil2cpp::ByRefMut<char>,
        decodingMap: quest_hook::libil2cpp::ByRefMut<i8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode", (encodedChars, decodingMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultToType(
        value: quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultToType", (value, targetType, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64CharArray(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBase64CharArray", (inArray, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64CharPtr(
        inputPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inputLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBase64CharPtr", (inputPtr, inputLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64String(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBase64String", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64_ComputeResultLength(
        inputPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inputLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBase64_ComputeResultLength", (inputPtr, inputLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSpace(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSpace", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowByteOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowByteOverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowCharOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowCharOverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt16OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInt16OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt32OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInt32OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt64OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInt64OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowSByteOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowSByteOverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt16OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowUInt16OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt32OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowUInt32OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt64OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowUInt64OverflowException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64CharArray_Base64FormattingOptions1(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offsetIn: i32,
        length: i32,
        outArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offsetOut: i32,
        options: crate::System::Base64FormattingOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToBase64CharArray",
                (inArray, offsetIn, length, outArray, offsetOut, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64CharArray_Il2CppArray_i32_i32_Il2CppArray_i32_0(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offsetIn: i32,
        length: i32,
        outArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offsetOut: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToBase64CharArray",
                (inArray, offsetIn, length, outArray, offsetOut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_Il2CppArray0(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBase64String", (inArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_Il2CppArray_i32_i32_1(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBase64String", (inArray, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_Il2CppArray_i32_i32_Base64FormattingOptions2(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
        options: crate::System::Base64FormattingOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBase64String", (inArray, offset, length, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_ReadOnlySpan_1_Base64FormattingOptions3(
        bytes: crate::System::ReadOnlySpan_1<u8>,
        options: crate::System::Base64FormattingOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBase64String", (bytes, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64_CalculateAndValidateOutputLength(
        inputLength: i32,
        insertLineBreaks: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToBase64_CalculateAndValidateOutputLength",
                (inputLength, insertLineBreaks),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Decimal14(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppString10(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_f32_12(value: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_f64_13(value: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i16_4(value: i16) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i32_6(value: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i64_8(value: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i8_2(value: i8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u16_5(value: u16) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u32_7(value: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u64_9(value: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u8_3(value: u8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i16_5(value: i16) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i32_7(value: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u16_6(value: u16) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u32_8(value: u32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppString10(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i16_4(value: i16) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i32_6(value: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i64_8(value: i64) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i8_2(value: i8) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u16_5(value: u16) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u32_7(value: u32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u64_9(value: u64) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u8_3(value: u8) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDateTime", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime_Il2CppString1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDateTime", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_Il2CppObject_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal__cordl_bool12(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_f32_9(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_f64_10(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i16_3(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i32_5(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i64_7(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i8_1(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u16_4(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u32_6(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u64_8(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u8_2(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Decimal11(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppString_IFormatProvider12(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble__cordl_bool13(value: bool) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_f32_10(value: f32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i16_4(value: i16) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i32_6(value: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i64_8(value: i64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i8_2(value: i8) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u16_5(value: u16) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u32_7(value: u32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u64_9(value: u64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u8_3(value: u8) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i32_7(value: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i64_9(value: i64) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i8_4(value: i8) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u32_8(value: u32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u8_5(value: u8) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Decimal12(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString13(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_f32_10(value: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_f64_11(value: f64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_i16_5(value: i16) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_i64_8(value: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u32_7(value: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u64_9(value: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u8_4(value: u8) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i16_6(value: i16) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i32_8(value: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i8_4(value: i8) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u16_7(value: u16) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u32_9(value: u32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u8_5(value: u8) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i16_5(value: i16) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i32_7(value: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i64_9(value: i64) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u32_8(value: u32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u8_4(value: u8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Decimal11(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppString12(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppString_IFormatProvider13(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle__cordl_bool14(value: bool) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_f64_10(value: f64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i16_4(value: i16) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i32_6(value: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i64_8(value: i64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i8_2(value: i8) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u16_5(value: u16) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u32_7(value: u32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u64_9(value: u64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u8_3(value: u8) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppObject_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool1(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_char_IFormatProvider2(
        value: char,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f32_6(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_3(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_IFormatProvider4(
        value: i32,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_i32_7(
        value: i32,
        toBase: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, toBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_5(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_i32_8(
        value: i64,
        toBase: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToString", (value, toBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i32_7(value: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u32_8(value: u32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i32_8(value: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u16_7(value: u16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value, fromBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i32_8(value: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i64_10(value: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u16_7(value: u16) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u32_9(value: u32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDecodeFromUtf16(
        utf16: crate::System::ReadOnlySpan_1<char>,
        bytes: crate::System::Span_1<u8>,
        consumed: quest_hook::libil2cpp::ByRefMut<i32>,
        written: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryDecodeFromUtf16", (utf16, bytes, consumed, written))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFromBase64Chars(
        chars: crate::System::ReadOnlySpan_1<char>,
        bytes: crate::System::Span_1<u8>,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFromBase64Chars", (chars, bytes, bytesWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteThreeLowOrderBytes(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteThreeLowOrderBytes", (destination, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Convert")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Convert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
