#[cfg(feature = "System+Guid")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Guid {
    pub _a: i32,
    pub _b: i16,
    pub _c: i16,
    pub _d: u8,
    pub _e: u8,
    pub _f: u8,
    pub _g: u8,
    pub _h: u8,
    pub _i: u8,
    pub _j: u8,
    pub _k: u8,
}
#[cfg(feature = "System+Guid")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid => "System"."Guid"
);
#[cfg(feature = "System+Guid")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Guid {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Guid")]
impl crate::System::Guid {
    #[cfg(feature = "System+Guid+GuidResult")]
    pub type GuidResult = crate::System::Guid_GuidResult;
    #[cfg(feature = "System+Guid+GuidParseThrowStyle")]
    pub type GuidParseThrowStyle = crate::System::Guid_GuidParseThrowStyle;
    #[cfg(feature = "System+Guid+ParseFailureKind")]
    pub type ParseFailureKind = crate::System::Guid_ParseFailureKind;
    #[cfg(feature = "System+Guid+GuidStyles")]
    pub type GuidStyles = crate::System::Guid_GuidStyles;
    pub fn GetResult(
        &mut self,
        me: u32,
        them: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResult",
            (me, them),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ReadOnlySpan_1_1(
        &mut self,
        b: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i16_i16_Il2CppArray2(
        &mut self,
        a: i32,
        b: i16,
        c: i16,
        d: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i16_i16_u8_u8_u8_u8_u8_u8_u8_u8_3(
        &mut self,
        a: i32,
        b: i16,
        c: i16,
        d: u8,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
        j: u8,
        k: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String4(
        &mut self,
        g: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (g),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToByteArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToByteArray",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ISpanFormattable_TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ISpanFormattable.TryFormat",
            (destination, charsWritten, format, provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Guid1(
        &mut self,
        g: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (g),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String1(
        &mut self,
        format: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider2(
        &mut self,
        format: *mut crate::System::String,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_Guid1(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WriteByteHelper(
        &mut self,
        destination: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteByteHelper",
            (destination),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Guid+GuidParseThrowStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guid_GuidParseThrowStyle {
    All = 1i32,
    AllButOverflow = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Guid+GuidParseThrowStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidParseThrowStyle => "System"
    ."Guid/GuidParseThrowStyle"
);
#[cfg(feature = "System+Guid+GuidResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Guid_GuidResult {
    pub _parsedGuid: crate::System::Guid,
    pub _throwStyle: crate::System::Guid_GuidParseThrowStyle,
    pub _failure: crate::System::Guid_ParseFailureKind,
    pub _failureMessageID: *mut crate::System::String,
    pub _failureMessageFormatArgument: *mut crate::System::Object,
    pub _failureArgumentName: *mut crate::System::String,
    pub _innerException: *mut crate::System::Exception,
}
#[cfg(feature = "System+Guid+GuidResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidResult => "System"
    ."Guid/GuidResult"
);
#[cfg(feature = "System+Guid+GuidResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Guid_GuidResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Guid+GuidResult")]
impl crate::System::Guid_GuidResult {
    pub fn Init(
        &mut self,
        canThrow: crate::System::Guid_GuidParseThrowStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (canThrow),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetGuidParseException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_ret: *mut crate::System::Exception = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetGuidParseException",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Exception0(
        &mut self,
        nativeException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (nativeException),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Guid_ParseFailureKind_String1(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Guid_ParseFailureKind_String_Object2(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: *mut crate::System::String,
        failureMessageFormatArgument: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID, failureMessageFormatArgument),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Guid_ParseFailureKind_String_Object_String_Exception3(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: *mut crate::System::String,
        failureMessageFormatArgument: *mut crate::System::Object,
        failureArgumentName: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (
                failure,
                failureMessageID,
                failureMessageFormatArgument,
                failureArgumentName,
                innerException,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Guid+GuidStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guid_GuidStyles {
    AllowBraces = 2i32,
    AllowDashes = 4i32,
    AllowHexPrefix = 8i32,
    AllowParenthesis = 1i32,
    Any = 15i32,
    BraceFormat = 96i32,
    DigitFormat = 64i32,
    HexFormat = 160i32,
    None = 0i32,
    ParenthesisFormat = 80i32,
    RequireBraces = 32i32,
    RequireHexPrefix = 128i32,
    RequireParenthesis = 16i32,
}
#[cfg(feature = "System+Guid+GuidStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidStyles => "System"
    ."Guid/GuidStyles"
);
#[cfg(feature = "System+Guid+ParseFailureKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guid_ParseFailureKind {
    ArgumentNull = 1i32,
    Format = 2i32,
    FormatWithInnerException = 5i32,
    FormatWithParameter = 3i32,
    NativeException = 4i32,
    None = 0i32,
}
#[cfg(feature = "System+Guid+ParseFailureKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_ParseFailureKind => "System"
    ."Guid/ParseFailureKind"
);
