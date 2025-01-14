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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("ChangeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, conversionType)) };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType_TypeCode_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        typeCode: crate::System::TypeCode,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::TypeCode,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("ChangeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeType", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, typeCode, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType_Type_IFormatProvider2(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        conversionType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("ChangeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeType", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, conversionType, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToBase64Array(
        outChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        offset: i32,
        length: i32,
        insertLineBreaks: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    bool,
                ),
                i32,
                5usize,
            >("ConvertToBase64Array")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertToBase64Array", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (outChars, inData, offset, length, insertLineBreaks),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyToTempBufferWithoutWhiteSpace(
        chars: crate::System::ReadOnlySpan_1<char>,
        tempBuffer: crate::System::Span_1<char>,
        consumed: quest_hook::libil2cpp::ByRefMut<i32>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("CopyToTempBufferWithoutWhiteSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyToTempBufferWithoutWhiteSpace", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (chars, tempBuffer, consumed, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        encodedChars: quest_hook::libil2cpp::ByRefMut<char>,
        decodingMap: quest_hook::libil2cpp::ByRefMut<i8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<char>,
                    quest_hook::libil2cpp::ByRefMut<i8>,
                ),
                i32,
                2usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Decode", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (encodedChars, decodingMap))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultToType(
        value: quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("DefaultToType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultToType", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (value, targetType, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64CharArray(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                3usize,
            >("FromBase64CharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBase64CharArray", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (inArray, offset, length)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64CharPtr(
        inputPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inputLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("FromBase64CharPtr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBase64CharPtr", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (inputPtr, inputLength)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64String(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("FromBase64String")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBase64String", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (s)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBase64_ComputeResultLength(
        inputPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inputLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                i32,
                2usize,
            >("FromBase64_ComputeResultLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBase64_ComputeResultLength", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (inputPtr, inputLength))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::System::TypeCode,
                1usize,
            >("GetTypeCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTypeCode", 1usize
                )
            });
        let __cordl_ret: crate::System::TypeCode = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSpace(c: char) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), bool, 1usize>("IsSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsSpace", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (c)) };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowByteOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowByteOverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowByteOverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowCharOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowCharOverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowCharOverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt16OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowInt16OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowInt16OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt32OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowInt32OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowInt32OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInt64OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowInt64OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowInt64OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowSByteOverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowSByteOverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowSByteOverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt16OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowUInt16OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowUInt16OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt32OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowUInt32OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowUInt32OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUInt64OverflowException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowUInt64OverflowException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowUInt64OverflowException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    i32,
                    crate::System::Base64FormattingOptions,
                ),
                i32,
                6usize,
            >("ToBase64CharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64CharArray", 6usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (inArray, offsetIn, length, outArray, offsetOut, options),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64CharArray_Il2CppArray_i32_i32_Il2CppArray_i32_0(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offsetIn: i32,
        length: i32,
        outArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        offsetOut: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    i32,
                ),
                i32,
                5usize,
            >("ToBase64CharArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64CharArray", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (inArray, offsetIn, length, outArray, offsetOut))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_Il2CppArray0(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToBase64String")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64String", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (inArray)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_Il2CppArray_i32_i32_1(
        inArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("ToBase64String")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64String", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (inArray, offset, length)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                    crate::System::Base64FormattingOptions,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("ToBase64String")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64String", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (inArray, offset, length, options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64String_ReadOnlySpan_1_Base64FormattingOptions3(
        bytes: crate::System::ReadOnlySpan_1<u8>,
        options: crate::System::Base64FormattingOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<u8>,
                    crate::System::Base64FormattingOptions,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToBase64String")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64String", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (bytes, options)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBase64_CalculateAndValidateOutputLength(
        inputLength: i32,
        insertLineBreaks: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, bool),
                i32,
                2usize,
            >("ToBase64_CalculateAndValidateOutputLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBase64_CalculateAndValidateOutputLength", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (inputLength, insertLineBreaks))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Decimal14(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                bool,
                2usize,
            >("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppString10(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                bool,
                2usize,
            >("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_f32_12(value: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_f64_13(value: f64) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i16_4(value: i16) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i32_6(value: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i64_8(value: i64) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_i8_2(value: i8) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u16_5(value: u16) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u32_7(value: u32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u64_9(value: u64) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToBoolean_u8_3(value: u8) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), bool, 1usize>("ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                u8,
                1usize,
            >("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u8,
                2usize,
            >("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u8,
                1usize,
            >("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u8,
                2usize,
            >("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                u8,
                2usize,
            >("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i16_5(value: i16) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i32_7(value: i32) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u16_6(value: u16) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u32_8(value: u32) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToByte_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u8, 1usize>("ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                char,
                1usize,
            >("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                char,
                2usize,
            >("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 2usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppString10(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                char,
                1usize,
            >("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                char,
                2usize,
            >("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 2usize
                )
            });
        let __cordl_ret: char = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i16_4(value: i16) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i32_6(value: i32) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i64_8(value: i64) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_i8_2(value: i8) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u16_5(value: u16) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u32_7(value: u32) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u64_9(value: u64) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToChar_u8_3(value: u8) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), char, 1usize>("ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::DateTime,
                2usize,
            >("ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDateTime", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime_Il2CppString1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::DateTime,
                2usize,
            >("ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDateTime", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_Il2CppObject_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::Decimal,
                2usize,
            >("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 2usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_Il2CppString_IFormatProvider11(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::Decimal,
                2usize,
            >("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 2usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal__cordl_bool12(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_f32_9(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_f64_10(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i16_3(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i32_5(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i64_7(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_i8_1(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u16_4(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u32_6(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u64_8(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal_u8_2(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), crate::System::Decimal, 1usize>("ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Decimal11(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                f64,
                1usize,
            >("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f64,
                2usize,
            >("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_Il2CppString_IFormatProvider12(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f64,
                2usize,
            >("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble__cordl_bool13(value: bool) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_f32_10(value: f32) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i16_4(value: i16) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i32_6(value: i32) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i64_8(value: i64) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_i8_2(value: i8) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u16_5(value: u16) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u32_7(value: u32) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u64_9(value: u64) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble_u8_3(value: u8) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), f64, 1usize>("ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i16,
                1usize,
            >("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i16,
                2usize,
            >("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i16,
                2usize,
            >("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                i16,
                2usize,
            >("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 2usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i32_7(value: i32) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i64_9(value: i64) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_i8_4(value: i8) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u32_8(value: u32) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16_u8_5(value: u8) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), i16, 1usize>("ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Decimal12(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i32,
                1usize,
            >("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i32,
                2usize,
            >("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString13(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i32,
                2usize,
            >("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                i32,
                2usize,
            >("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_f32_10(value: f32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_f64_11(value: f64) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_i16_5(value: i16) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_i64_8(value: i64) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u32_7(value: u32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u64_9(value: u64) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32_u8_4(value: u8) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), i32, 1usize>("ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i64,
                1usize,
            >("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i64,
                2usize,
            >("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i64,
                1usize,
            >("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i64,
                2usize,
            >("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                i64,
                2usize,
            >("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i16_6(value: i16) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i32_8(value: i32) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_i8_4(value: i8) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u16_7(value: u16) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u32_9(value: u32) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64_u8_5(value: u8) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), i64, 1usize>("ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i8,
                1usize,
            >("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i8,
                2usize,
            >("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                i8,
                2usize,
            >("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                i8,
                2usize,
            >("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 2usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_f32_11(value: f32) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_f64_12(value: f64) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i16_5(value: i16) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i32_7(value: i32) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_i64_9(value: i64) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u16_6(value: u16) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u32_8(value: u32) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u64_10(value: u64) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte_u8_4(value: u8) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), i8, 1usize>("ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Decimal11(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                f32,
                1usize,
            >("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f32,
                2usize,
            >("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppString12(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_Il2CppString_IFormatProvider13(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f32,
                2usize,
            >("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle__cordl_bool14(value: bool) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_f64_10(value: f64) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i16_4(value: i16) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i32_6(value: i32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i64_8(value: i64) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_i8_2(value: i8) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u16_5(value: u16) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u32_7(value: u32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u64_9(value: u64) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle_u8_3(value: u8) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), f32, 1usize>("ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppObject_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_bool1(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString__cordl_char_IFormatProvider2(
        value: char,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (char, quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_f32_6(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_3(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_IFormatProvider4(
        value: i32,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_i32_7(
        value: i32,
        toBase: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, toBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_5(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i64_i32_8(
        value: i64,
        toBase: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, toBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                u16,
                1usize,
            >("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u16,
                2usize,
            >("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u16,
                2usize,
            >("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                u16,
                2usize,
            >("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i32_7(value: i32) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u32_8(value: u32) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), u16, 1usize>("ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                u32,
                1usize,
            >("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u32,
                2usize,
            >("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppString_IFormatProvider14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u32,
                2usize,
            >("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_Il2CppString_i32_15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                u32,
                2usize,
            >("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i32_8(value: i32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i64_9(value: i64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u16_7(value: u16) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u64_10(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), u32, 1usize>("ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Decimal13(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::Decimal), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppObject0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                u64,
                1usize,
            >("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppObject_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u64,
                2usize,
            >("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString14(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u64,
                1usize,
            >("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString_IFormatProvider15(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                u64,
                2usize,
            >("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_Il2CppString_i32_16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fromBase: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                u64,
                2usize,
            >("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value, fromBase)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64__cordl_bool2(value: bool) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64__cordl_char3(value: char) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_f32_11(value: f32) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_f64_12(value: f64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i16_6(value: i16) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i16), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i32_8(value: i32) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i64_10(value: i64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_i8_4(value: i8) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i8), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u16_7(value: u16) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u32_9(value: u32) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64_u8_5(value: u8) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u8), u64, 1usize>("ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryDecodeFromUtf16(
        utf16: crate::System::ReadOnlySpan_1<char>,
        bytes: crate::System::Span_1<u8>,
        consumed: quest_hook::libil2cpp::ByRefMut<i32>,
        written: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Span_1<u8>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("TryDecodeFromUtf16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryDecodeFromUtf16", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (utf16, bytes, consumed, written))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFromBase64Chars(
        chars: crate::System::ReadOnlySpan_1<char>,
        bytes: crate::System::Span_1<u8>,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Span_1<u8>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("TryFromBase64Chars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFromBase64Chars", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (chars, bytes, bytesWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteThreeLowOrderBytes(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u8>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteThreeLowOrderBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteThreeLowOrderBytes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (destination, value))
        };
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
