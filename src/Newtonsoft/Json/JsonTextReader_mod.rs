#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonTextReader {
    __cordl_parent: crate::Newtonsoft::Json::JsonReader,
    pub _safeAsync: bool,
    pub _reader: *mut crate::System::IO::TextReader,
    pub _chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _charsUsed: i32,
    pub _charPos: i32,
    pub _lineStartPos: i32,
    pub _lineNumber: i32,
    pub _isEndOfFile: bool,
    pub _stringBuffer: crate::Newtonsoft::Json::Utilities::StringBuffer,
    pub _stringReference: crate::Newtonsoft::Json::Utilities::StringReference,
    pub _arrayPool: *mut crate::Newtonsoft::Json::IArrayPool_1<char>,
    pub _PropertyNameTable_k__BackingField: *mut crate::Newtonsoft::Json::JsonNameTable,
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonTextReader =>
    "Newtonsoft.Json"."JsonTextReader"
);
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonTextReader {
    type Target = crate::Newtonsoft::Json::JsonReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonTextReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl crate::Newtonsoft::Json::JsonTextReader {
    pub const LargeBufferLength: i32 = 1073741823i32;
    pub const MaximumJavascriptIntegerCharacterLength: i32 = 380i32;
    pub const UnicodeReplacementChar: char = '\u{fffd}';
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsBooleanAsync_d__40")]
    pub type _DoReadAsBooleanAsync_d__40 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsBooleanAsync_d__40;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsBytesAsync_d__42")]
    pub type _DoReadAsBytesAsync_d__42 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsBytesAsync_d__42;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsDateTimeAsync_d__45")]
    pub type _DoReadAsDateTimeAsync_d__45 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsDateTimeAsync_d__45;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsDateTimeOffsetAsync_d__47")]
    pub type _DoReadAsDateTimeOffsetAsync_d__47 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsDateTimeOffsetAsync_d__47;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsDecimalAsync_d__49")]
    pub type _DoReadAsDecimalAsync_d__49 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsDecimalAsync_d__49;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsDoubleAsync_d__51")]
    pub type _DoReadAsDoubleAsync_d__51 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsDoubleAsync_d__51;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsInt32Async_d__53")]
    pub type _DoReadAsInt32Async_d__53 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsInt32Async_d__53;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsStringAsync_d__55")]
    pub type _DoReadAsStringAsync_d__55 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsStringAsync_d__55;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_DoReadAsync_d__3")]
    pub type _DoReadAsync_d__3 = crate::Newtonsoft::Json::JsonTextReader__DoReadAsync_d__3;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_EatWhitespaceAsync_d__17")]
    pub type _EatWhitespaceAsync_d__17 = crate::Newtonsoft::Json::JsonTextReader__EatWhitespaceAsync_d__17;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_HandleNullAsync_d__35")]
    pub type _HandleNullAsync_d__35 = crate::Newtonsoft::Json::JsonTextReader__HandleNullAsync_d__35;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_MatchAndSetAsync_d__21")]
    pub type _MatchAndSetAsync_d__21 = crate::Newtonsoft::Json::JsonTextReader__MatchAndSetAsync_d__21;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_MatchValueAsync_d__19")]
    pub type _MatchValueAsync_d__19 = crate::Newtonsoft::Json::JsonTextReader__MatchValueAsync_d__19;
    #[cfg(
        feature = "Newtonsoft+Json+JsonTextReader+_MatchValueWithTrailingSeparatorAsync_d__20"
    )]
    pub type _MatchValueWithTrailingSeparatorAsync_d__20 = crate::Newtonsoft::Json::JsonTextReader__MatchValueWithTrailingSeparatorAsync_d__20;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseCommentAsync_d__16")]
    pub type _ParseCommentAsync_d__16 = crate::Newtonsoft::Json::JsonTextReader__ParseCommentAsync_d__16;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseConstructorAsync_d__25")]
    pub type _ParseConstructorAsync_d__25 = crate::Newtonsoft::Json::JsonTextReader__ParseConstructorAsync_d__25;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseNumberAsync_d__29")]
    pub type _ParseNumberAsync_d__29 = crate::Newtonsoft::Json::JsonTextReader__ParseNumberAsync_d__29;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseNumberNaNAsync_d__26")]
    pub type _ParseNumberNaNAsync_d__26 = crate::Newtonsoft::Json::JsonTextReader__ParseNumberNaNAsync_d__26;
    #[cfg(
        feature = "Newtonsoft+Json+JsonTextReader+_ParseNumberNegativeInfinityAsync_d__28"
    )]
    pub type _ParseNumberNegativeInfinityAsync_d__28 = crate::Newtonsoft::Json::JsonTextReader__ParseNumberNegativeInfinityAsync_d__28;
    #[cfg(
        feature = "Newtonsoft+Json+JsonTextReader+_ParseNumberPositiveInfinityAsync_d__27"
    )]
    pub type _ParseNumberPositiveInfinityAsync_d__27 = crate::Newtonsoft::Json::JsonTextReader__ParseNumberPositiveInfinityAsync_d__27;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseObjectAsync_d__15")]
    pub type _ParseObjectAsync_d__15 = crate::Newtonsoft::Json::JsonTextReader__ParseObjectAsync_d__15;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParsePostValueAsync_d__4")]
    pub type _ParsePostValueAsync_d__4 = crate::Newtonsoft::Json::JsonTextReader__ParsePostValueAsync_d__4;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParsePropertyAsync_d__31")]
    pub type _ParsePropertyAsync_d__31 = crate::Newtonsoft::Json::JsonTextReader__ParsePropertyAsync_d__31;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseStringAsync_d__18")]
    pub type _ParseStringAsync_d__18 = crate::Newtonsoft::Json::JsonTextReader__ParseStringAsync_d__18;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseUnicodeAsync_d__12")]
    pub type _ParseUnicodeAsync_d__12 = crate::Newtonsoft::Json::JsonTextReader__ParseUnicodeAsync_d__12;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseUnquotedPropertyAsync_d__33")]
    pub type _ParseUnquotedPropertyAsync_d__33 = crate::Newtonsoft::Json::JsonTextReader__ParseUnquotedPropertyAsync_d__33;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ParseValueAsync_d__8")]
    pub type _ParseValueAsync_d__8 = crate::Newtonsoft::Json::JsonTextReader__ParseValueAsync_d__8;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ProcessCarriageReturnAsync_d__11")]
    pub type _ProcessCarriageReturnAsync_d__11 = crate::Newtonsoft::Json::JsonTextReader__ProcessCarriageReturnAsync_d__11;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadCharsAsync_d__14")]
    pub type _ReadCharsAsync_d__14 = crate::Newtonsoft::Json::JsonTextReader__ReadCharsAsync_d__14;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadDataAsync_d__7")]
    pub type _ReadDataAsync_d__7 = crate::Newtonsoft::Json::JsonTextReader__ReadDataAsync_d__7;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadFinishedAsync_d__36")]
    pub type _ReadFinishedAsync_d__36 = crate::Newtonsoft::Json::JsonTextReader__ReadFinishedAsync_d__36;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadFromFinishedAsync_d__5")]
    pub type _ReadFromFinishedAsync_d__5 = crate::Newtonsoft::Json::JsonTextReader__ReadFromFinishedAsync_d__5;
    #[cfg(
        feature = "Newtonsoft+Json+JsonTextReader+_ReadIntoWrappedTypeObjectAsync_d__43"
    )]
    pub type _ReadIntoWrappedTypeObjectAsync_d__43 = crate::Newtonsoft::Json::JsonTextReader__ReadIntoWrappedTypeObjectAsync_d__43;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadNullCharAsync_d__34")]
    pub type _ReadNullCharAsync_d__34 = crate::Newtonsoft::Json::JsonTextReader__ReadNullCharAsync_d__34;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadNumberIntoBufferAsync_d__32")]
    pub type _ReadNumberIntoBufferAsync_d__32 = crate::Newtonsoft::Json::JsonTextReader__ReadNumberIntoBufferAsync_d__32;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadNumberValueAsync_d__38")]
    pub type _ReadNumberValueAsync_d__38 = crate::Newtonsoft::Json::JsonTextReader__ReadNumberValueAsync_d__38;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadStringIntoBufferAsync_d__9")]
    pub type _ReadStringIntoBufferAsync_d__9 = crate::Newtonsoft::Json::JsonTextReader__ReadStringIntoBufferAsync_d__9;
    #[cfg(feature = "Newtonsoft+Json+JsonTextReader+_ReadStringValueAsync_d__37")]
    pub type _ReadStringValueAsync_d__37 = crate::Newtonsoft::Json::JsonTextReader__ReadStringValueAsync_d__37;
    pub fn ClearRecentString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRecentString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUnicode(
        &mut self,
        enoughChars: bool,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ConvertUnicode", (enoughChars))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnexpectedCharacterException(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReaderException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonReaderException,
        > = __cordl_object.invoke("CreateUnexpectedCharacterException", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsBooleanAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<bool>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<bool>>,
        > = __cordl_object.invoke("DoReadAsBooleanAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsBytesAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        > = __cordl_object.invoke("DoReadAsBytesAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsDateTimeAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTime>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTime>,
            >,
        > = __cordl_object.invoke("DoReadAsDateTimeAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsDateTimeOffsetAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTimeOffset>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTimeOffset>,
            >,
        > = __cordl_object.invoke("DoReadAsDateTimeOffsetAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsDecimalAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::Decimal>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::Decimal>,
            >,
        > = __cordl_object.invoke("DoReadAsDecimalAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsDoubleAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<f64>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<f64>>,
        > = __cordl_object.invoke("DoReadAsDoubleAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsInt32Async(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        > = __cordl_object.invoke("DoReadAsInt32Async", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsStringAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("DoReadAsStringAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsync_CancellationToken0(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("DoReadAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoReadAsync_Task_1_CancellationToken1(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("DoReadAsync", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn EatWhitespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EatWhitespace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EatWhitespaceAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("EatWhitespaceAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndComment(
        &mut self,
        setToken: bool,
        initialPosition: i32,
        endPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndComment", (setToken, initialPosition, endPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureBufferNotEmpty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureBufferNotEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureChars(
        &mut self,
        relativePosition: i32,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EnsureChars", (relativePosition, append))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureCharsAsync(
        &mut self,
        relativePosition: i32,
        append: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("EnsureCharsAsync", (relativePosition, append, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishReadQuotedNumber(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("FinishReadQuotedNumber", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishReadQuotedStringValue(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("FinishReadQuotedStringValue", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishReadStringIntoBuffer(
        &mut self,
        charPos: i32,
        initialPosition: i32,
        lastWritePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FinishReadStringIntoBuffer",
                (charPos, initialPosition, lastWritePosition),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNullAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("HandleNullAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSeparator(&mut self, c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSeparator", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchAndSetAsync(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newToken: crate::Newtonsoft::Json::JsonToken,
        tokenValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "MatchAndSetAsync",
                (value, newToken, tokenValue, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchValueAsync(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("MatchValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchValueWithTrailingSeparator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchValueWithTrailingSeparator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchValueWithTrailingSeparatorAsync(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("MatchValueWithTrailingSeparatorAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchValue_Il2CppString0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchValue__cordl_bool_Il2CppString1(
        &mut self,
        enoughChars: bool,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchValue", (enoughChars, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object.into())
    }
    pub fn OnNewLine(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewLine", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseComment(
        &mut self,
        setToken: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseComment", (setToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCommentAsync(
        &mut self,
        setToken: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseCommentAsync", (setToken, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseConstructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseConstructorAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseConstructorAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFalse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseFalse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFalseAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseFalseAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNullAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseNullAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumber(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseNumber", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseNumberAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNaNAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("ParseNumberNaNAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNaN_ReadType0(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberNaN", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNaN__cordl_bool1(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        matched: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberNaN", (readType, matched))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNegativeInfinityAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object
            .invoke("ParseNumberNegativeInfinityAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNegativeInfinity_ReadType0(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberNegativeInfinity", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberNegativeInfinity__cordl_bool1(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        matched: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberNegativeInfinity", (readType, matched))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberPositiveInfinityAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object
            .invoke("ParseNumberPositiveInfinityAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberPositiveInfinity_ReadType0(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberPositiveInfinity", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumberPositiveInfinity__cordl_bool1(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        matched: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ParseNumberPositiveInfinity", (readType, matched))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseObjectAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ParseObjectAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePostValue(
        &mut self,
        ignoreComments: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParsePostValue", (ignoreComments))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePostValueAsync(
        &mut self,
        ignoreComments: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("ParsePostValueAsync", (ignoreComments, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseProperty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseProperty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePropertyAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ParsePropertyAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseReadNumber(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        firstChar: char,
        initialPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseReadNumber", (readType, firstChar, initialPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseReadString(
        &mut self,
        quote: char,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseReadString", (quote, readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseString(
        &mut self,
        quote: char,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseString", (quote, readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseStringAsync(
        &mut self,
        quote: char,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("ParseStringAsync", (quote, readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTrue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseTrue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTrueAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseTrueAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUndefined(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseUndefined", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUndefinedAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseUndefinedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnicode(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ParseUnicode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnicodeAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<char>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<char>,
        > = __cordl_object.invoke("ParseUnicodeAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnquotedProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseUnquotedProperty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnquotedPropertyAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ParseUnquotedPropertyAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseValueAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ParseValueAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareBufferForReadData(
        &mut self,
        append: bool,
        charsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareBufferForReadData", (append, charsRequired))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCarriageReturn(
        &mut self,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCarriageReturn", (append))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCarriageReturnAsync_Task_1_1(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ProcessCarriageReturnAsync", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCarriageReturnAsync__cordl_bool_CancellationToken0(
        &mut self,
        append: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("ProcessCarriageReturnAsync", (append, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLineFeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessLineFeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessValueComma(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessValueComma", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsBoolean(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("ReadAsBoolean", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsBooleanAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<bool>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<bool>>,
        > = __cordl_object.invoke("ReadAsBooleanAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadAsBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsBytesAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        > = __cordl_object.invoke("ReadAsBytesAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTime>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTime> = __cordl_object
            .invoke("ReadAsDateTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDateTimeAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTime>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTime>,
            >,
        > = __cordl_object.invoke("ReadAsDateTimeAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDateTimeOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTimeOffset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTimeOffset> = __cordl_object
            .invoke("ReadAsDateTimeOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDateTimeOffsetAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTimeOffset>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::DateTimeOffset>,
            >,
        > = __cordl_object.invoke("ReadAsDateTimeOffsetAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::Decimal>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::Decimal> = __cordl_object
            .invoke("ReadAsDecimal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDecimalAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::Decimal>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::Nullable_1<crate::System::Decimal>,
            >,
        > = __cordl_object.invoke("ReadAsDecimalAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDouble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("ReadAsDouble", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsDoubleAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<f64>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<f64>>,
        > = __cordl_object.invoke("ReadAsDoubleAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsInt32(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("ReadAsInt32", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsInt32Async(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        > = __cordl_object.invoke("ReadAsInt32Async", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadAsString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsStringAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("ReadAsStringAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ReadAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadChars(
        &mut self,
        relativePosition: i32,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadChars", (relativePosition, append))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCharsAsync(
        &mut self,
        relativePosition: i32,
        append: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("ReadCharsAsync", (relativePosition, append, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDataAsync_CancellationToken0(
        &mut self,
        append: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object.invoke("ReadDataAsync", (append, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDataAsync_i32_CancellationToken1(
        &mut self,
        append: bool,
        charsRequired: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object
            .invoke("ReadDataAsync", (append, charsRequired, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadData__cordl_bool0(
        &mut self,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadData", (append))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadData_i32_1(
        &mut self,
        append: bool,
        charsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadData", (append, charsRequired))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFinishedAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ReadFinishedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFromFinishedAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ReadFromFinishedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadIntoWrappedTypeObjectAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("ReadIntoWrappedTypeObjectAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNullChar(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadNullChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNullCharAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("ReadNullCharAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNumberCharIntoBuffer(
        &mut self,
        currentChar: char,
        charPos: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadNumberCharIntoBuffer", (currentChar, charPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNumberIntoBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadNumberIntoBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNumberIntoBufferAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ReadNumberIntoBufferAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNumberValue(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadNumberValue", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadNumberValueAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object
            .invoke("ReadNumberValueAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringIntoBuffer(
        &mut self,
        quote: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadStringIntoBuffer", (quote))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringIntoBufferAsync(
        &mut self,
        quote: char,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("ReadStringIntoBufferAsync", (quote, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringValue(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadStringValue", (readType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringValueAsync(
        &mut self,
        readType: crate::Newtonsoft::Json::ReadType,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object
            .invoke("ReadStringValueAsync", (readType, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnquotedPropertyReportIfDone(
        &mut self,
        currentChar: char,
        initialPosition: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadUnquotedPropertyReportIfDone", (currentChar, initialPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNewLine(
        &mut self,
        hasNextChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNewLine", (hasNextChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftBufferIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShiftBufferIfNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowReaderError(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReaderException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonReaderException,
        > = __cordl_object.invoke("ThrowReaderError", (message, ex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidIdentifierChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidIdentifierChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCharToBuffer(
        &mut self,
        writeChar: char,
        lastWritePosition: i32,
        writeToPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteCharToBuffer",
                (writeChar, lastWritePosition, writeToPosition),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArrayPool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::IArrayPool_1<char>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        > = __cordl_object.invoke("get_ArrayPool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonNameTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonNameTable,
        > = __cordl_object.invoke("get_PropertyNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ArrayPool(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::IArrayPool_1<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ArrayPool", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PropertyNameTable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PropertyNameTable", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonTextReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl AsRef<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::JsonTextReader {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonTextReader")]
impl AsMut<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::JsonTextReader {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
