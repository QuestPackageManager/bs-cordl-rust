#[cfg(feature = "Newtonsoft+Json+JsonReader")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonReader {
    __cordl_parent: crate::System::Object,
    pub _tokenType: crate::Newtonsoft::Json::JsonToken,
    pub _value: *mut crate::System::Object,
    pub _quoteChar: char,
    pub _currentState: crate::Newtonsoft::Json::JsonReader_State,
    pub _currentPosition: crate::Newtonsoft::Json::JsonPosition,
    pub _culture: *mut crate::System::Globalization::CultureInfo,
    pub _dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
    pub _maxDepth: crate::System::Nullable_1<i32>,
    pub _hasExceededMaxDepth: bool,
    pub _dateParseHandling: crate::Newtonsoft::Json::DateParseHandling,
    pub _floatParseHandling: crate::Newtonsoft::Json::FloatParseHandling,
    pub _dateFormatString: *mut crate::System::String,
    pub _stack: *mut crate::System::Collections::Generic::List_1<
        crate::Newtonsoft::Json::JsonPosition,
    >,
    pub _CloseInput_k__BackingField: bool,
    pub _SupportMultipleContent_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+JsonReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonReader =>
    "Newtonsoft.Json"."JsonReader"
);
#[cfg(feature = "Newtonsoft+Json+JsonReader")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonReader")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonReader")]
impl crate::Newtonsoft::Json::JsonReader {
    #[cfg(feature = "Newtonsoft+Json+JsonReader+State")]
    pub type State = crate::Newtonsoft::Json::JsonReader_State;
    #[cfg(feature = "Newtonsoft+Json+JsonReader+_ReadAndMoveToContentAsync_d__12")]
    pub type _ReadAndMoveToContentAsync_d__12 = crate::Newtonsoft::Json::JsonReader__ReadAndMoveToContentAsync_d__12;
    #[cfg(
        feature = "Newtonsoft+Json+JsonReader+_MoveToContentFromNonContentAsync_d__14"
    )]
    pub type _MoveToContentFromNonContentAsync_d__14 = crate::Newtonsoft::Json::JsonReader__MoveToContentFromNonContentAsync_d__14;
    #[cfg(feature = "Newtonsoft+Json+JsonReader+_ReadArrayIntoByteArrayAsync_d__5")]
    pub type _ReadArrayIntoByteArrayAsync_d__5 = crate::Newtonsoft::Json::JsonReader__ReadArrayIntoByteArrayAsync_d__5;
    #[cfg(feature = "Newtonsoft+Json+JsonReader+_SkipAsync_d__1")]
    pub type _SkipAsync_d__1 = crate::Newtonsoft::Json::JsonReader__SkipAsync_d__1;
    #[cfg(feature = "Newtonsoft+Json+JsonReader+_ReaderReadAndAssertAsync_d__2")]
    pub type _ReaderReadAndAssertAsync_d__2 = crate::Newtonsoft::Json::JsonReader__ReaderReadAndAssertAsync_d__2;
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnexpectedEndException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::JsonReaderException,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::JsonReaderException = __cordl_object
            .invoke("CreateUnexpectedEndException", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn GetContentToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonToken = __cordl_object
            .invoke("GetContentToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPosition(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonPosition> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonPosition = __cordl_object
            .invoke("GetPosition", (depth))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeForCloseToken(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonContainerType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonContainerType = __cordl_object
            .invoke("GetTypeForCloseToken", (token))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToContentAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("MoveToContentAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToContentFromNonContentAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("MoveToContentFromNonContentAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Peek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonContainerType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonContainerType = __cordl_object
            .invoke("Peek", ())?;
        Ok(__cordl_ret)
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonContainerType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonContainerType = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
        value: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAndAssert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAndAssert", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAndMoveToContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadAndMoveToContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAndMoveToContentAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("ReadAndMoveToContentAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayElementIntoByteArrayReportDone(
        &mut self,
        buffer: *mut crate::System::Collections::Generic::List_1<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadArrayElementIntoByteArrayReportDone", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayIntoByteArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadArrayIntoByteArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayIntoByteArrayAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadArrayIntoByteArrayAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsBoolean(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("ReadAsBoolean", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsBooleanAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<bool>,
        > = __cordl_object.invoke("ReadAsBooleanAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadAsBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsBytesAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadAsBytesAsync", (cancellationToken))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ReadAsDateTimeAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::DateTime>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::DateTime>,
        > = __cordl_object.invoke("ReadAsDateTimeAsync", (cancellationToken))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ReadAsDateTimeOffsetAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::DateTimeOffset>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::DateTimeOffset>,
        > = __cordl_object.invoke("ReadAsDateTimeOffsetAsync", (cancellationToken))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ReadAsDecimalAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::Decimal>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<crate::System::Decimal>,
        > = __cordl_object.invoke("ReadAsDecimalAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsDouble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("ReadAsDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsDoubleAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<f64>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<f64>,
        > = __cordl_object.invoke("ReadAsDoubleAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsInt32(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("ReadAsInt32", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsInt32Async(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<i32>,
        > = __cordl_object.invoke("ReadAsInt32Async", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadAsString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsStringAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("ReadAsStringAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("ReadAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBooleanString(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("ReadBooleanString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDateTimeOffsetString(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTimeOffset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTimeOffset> = __cordl_object
            .invoke("ReadDateTimeOffsetString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDateTimeString(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTime>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTime> = __cordl_object
            .invoke("ReadDateTimeString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDecimalString(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::Decimal>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::Decimal> = __cordl_object
            .invoke("ReadDecimalString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDoubleString(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f64> = __cordl_object
            .invoke("ReadDoubleString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadForType(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        hasConverter: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadForType", (contract, hasConverter))?;
        Ok(__cordl_ret)
    }
    pub fn ReadForTypeAndAssert(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        hasConverter: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadForTypeAndAssert", (contract, hasConverter))?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt32String(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("ReadInt32String", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ReadIntoWrappedTypeObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadIntoWrappedTypeObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReaderReadAndAssert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReaderReadAndAssert", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReaderReadAndAssertAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ReaderReadAndAssertAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn SetFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPostValueState(
        &mut self,
        updateIndex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPostValueState", (updateIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SetStateBasedOnCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStateBasedOnCurrent", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetToken_JsonToken0(
        &mut self,
        newToken: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (newToken))?;
        Ok(__cordl_ret)
    }
    pub fn SetToken_Object1(
        &mut self,
        newToken: crate::Newtonsoft::Json::JsonToken,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (newToken, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetToken_Object__cordl_bool2(
        &mut self,
        newToken: crate::Newtonsoft::Json::JsonToken,
        value: *mut crate::System::Object,
        updateIndex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (newToken, value, updateIndex))?;
        Ok(__cordl_ret)
    }
    pub fn Skip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Skip", ())?;
        Ok(__cordl_ret)
    }
    pub fn SkipAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SkipAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScopeWithFinishedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScopeWithFinishedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEnd(
        &mut self,
        endToken: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateEnd", (endToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CloseInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CloseInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Culture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("get_Culture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonReader_State> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonReader_State = __cordl_object
            .invoke("get_CurrentState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateFormatString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DateFormatString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateParseHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateParseHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateParseHandling = __cordl_object
            .invoke("get_DateParseHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTimeZoneHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateTimeZoneHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateTimeZoneHandling = __cordl_object
            .invoke("get_DateTimeZoneHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FloatParseHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::FloatParseHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::FloatParseHandling = __cordl_object
            .invoke("get_FloatParseHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxDepth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MaxDepth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Path", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_QuoteChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_QuoteChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SupportMultipleContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportMultipleContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TokenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonToken = __cordl_object
            .invoke("get_TokenType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CloseInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CloseInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Culture(
        &mut self,
        value: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Culture", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateFormatString(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateFormatString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateParseHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateParseHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateParseHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateTimeZoneHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateTimeZoneHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_FloatParseHandling(
        &mut self,
        value: crate::Newtonsoft::Json::FloatParseHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FloatParseHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MaxDepth(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxDepth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_QuoteChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_QuoteChar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SupportMultipleContent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SupportMultipleContent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonReader+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonReader_State {
    Array = 6i32,
    ArrayStart = 5i32,
    Closed = 7i32,
    Complete = 1i32,
    Constructor = 10i32,
    ConstructorStart = 9i32,
    Error = 11i32,
    Finished = 12i32,
    Object = 4i32,
    ObjectStart = 3i32,
    PostValue = 8i32,
    Property = 2i32,
    Start = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+JsonReader+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonReader_State =>
    "Newtonsoft.Json"."JsonReader/State"
);