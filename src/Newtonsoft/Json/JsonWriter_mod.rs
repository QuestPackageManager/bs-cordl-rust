#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _stack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::Newtonsoft::Json::JsonPosition,
        >,
    >,
    pub _currentPosition: crate::Newtonsoft::Json::JsonPosition,
    pub _currentState: crate::Newtonsoft::Json::JsonWriter_State,
    pub _formatting: crate::Newtonsoft::Json::Formatting,
    pub _CloseOutput_k__BackingField: bool,
    pub _AutoCompleteOnClose_k__BackingField: bool,
    pub _dateFormatHandling: crate::Newtonsoft::Json::DateFormatHandling,
    pub _dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
    pub _stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
    pub _floatFormatHandling: crate::Newtonsoft::Json::FloatFormatHandling,
    pub _dateFormatString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::JsonWriter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter";
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl crate::Newtonsoft::Json::JsonWriter {
    #[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
    pub type State = crate::Newtonsoft::Json::JsonWriter_State;
    #[cfg(
        feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d"
    )]
    pub type __InternalWriteEndAsync_g__AwaitEnd_11_2_d = crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d;
    #[cfg(
        feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
    )]
    pub type __InternalWriteEndAsync_g__AwaitIndent_11_1_d = crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d;
    #[cfg(
        feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
    )]
    pub type __InternalWriteEndAsync_g__AwaitProperty_11_0_d = crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d;
    #[cfg(
        feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
    )]
    pub type __InternalWriteEndAsync_g__AwaitRemaining_11_3_d = crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d;
    pub fn AutoComplete(
        &mut self,
        tokenBeingWritten: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoComplete", (tokenBeingWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoCompleteAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoCompleteAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoCompleteAsync_CancellationToken1(
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
        > = __cordl_object.invoke("AutoCompleteAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoCompleteAsync_JsonToken_CancellationToken0(
        &mut self,
        tokenBeingWritten: crate::Newtonsoft::Json::JsonToken,
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
            .invoke("AutoCompleteAsync", (tokenBeingWritten, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoCompleteClose(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoCompleteClose", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildStateArray() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::Newtonsoft::Json::JsonWriter_State,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::Newtonsoft::Json::JsonWriter_State,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildStateArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLevelsToComplete(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateLevelsToComplete", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateWriteTokenFinalDepth(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateWriteTokenFinalDepth", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateWriteTokenInitialDepth(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateWriteTokenInitialDepth", (reader))?;
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
    pub fn CloseAsync(
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
        > = __cordl_object.invoke("CloseAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnsupportedTypeException(
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriterException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonWriterException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUnsupportedTypeException", (writer, value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsync(
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
        > = __cordl_object.invoke("FlushAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCloseTokenForType(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonToken = __cordl_object
            .invoke("GetCloseTokenForType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteComment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteComment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteCommentAsync(
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
        > = __cordl_object.invoke("InternalWriteCommentAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteEnd(
        &mut self,
        container: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteEnd", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteEndAsync(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
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
            .invoke("InternalWriteEndAsync", (_cordl_type, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWritePropertyName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWritePropertyName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWritePropertyNameAsync(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
            .invoke("InternalWritePropertyNameAsync", (name, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteRaw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteRaw", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteStart(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        container: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteStart", (token, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteStartAsync(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        container: crate::Newtonsoft::Json::JsonContainerType,
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
            .invoke("InternalWriteStartAsync", (token, container, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteValue(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteValue", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteValueAsync(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
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
            .invoke("InternalWriteValueAsync", (token, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteWhitespace(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteWhitespace", (ws))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWriteTokenIncomplete(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        writeChildren: bool,
        initialDepth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsWriteTokenIncomplete", (reader, writeChildren, initialDepth))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnStringEscapeHandlingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStringEscapeHandlingChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Peek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonContainerType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonContainerType = __cordl_object
            .invoke("Peek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Pop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonContainerType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonContainerType = __cordl_object
            .invoke("Pop", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ResolveConvertibleValue(
        convertible: quest_hook::libil2cpp::Gc<crate::System::IConvertible>,
        typeCode: quest_hook::libil2cpp::ByRefMut<
            crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveConvertibleValue", (convertible, typeCode, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWriteState(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWriteState", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWriteStateAsync(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
            .invoke("SetWriteStateAsync", (token, value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCurrentState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCurrentState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScopeWithFinishedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScopeWithFinishedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteComment(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteComment", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCommentAsync(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteCommentAsync", (text, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteConstructorDate(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteConstructorDate", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteConstructorDateAsync(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
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
            .invoke("WriteConstructorDateAsync", (reader, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndArrayAsync(
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
        > = __cordl_object.invoke("WriteEndArrayAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndAsync_CancellationToken1(
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
        > = __cordl_object.invoke("WriteEndAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndAsync_JsonToken_CancellationToken0(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteEndAsync", (token, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndConstructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndConstructorAsync(
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
        > = __cordl_object.invoke("WriteEndConstructorAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndInternalAsync(
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
        > = __cordl_object.invoke("WriteEndInternalAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndObjectAsync(
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
        > = __cordl_object.invoke("WriteEndObjectAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEnd_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEnd_JsonContainerType1(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnd", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEnd_JsonToken2(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEnd", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIndent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteIndent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIndentAsync(
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
        > = __cordl_object.invoke("WriteIndentAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIndentSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteIndentSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIndentSpaceAsync(
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
        > = __cordl_object.invoke("WriteIndentSpaceAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteNullAsync(
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
        > = __cordl_object.invoke("WriteNullAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePropertyNameAsync_CancellationToken0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WritePropertyNameAsync", (name, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePropertyNameAsync__cordl_bool_CancellationToken1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        escape: bool,
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
            .invoke("WritePropertyNameAsync", (name, escape, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePropertyName_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePropertyName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePropertyName__cordl_bool1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        escape: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePropertyName", (name, escape))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw(
        &mut self,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRawAsync(
        &mut self,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteRawAsync", (json, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRawValue(
        &mut self,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRawValue", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRawValueAsync(
        &mut self,
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteRawValueAsync", (json, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartArrayAsync(
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
        > = __cordl_object.invoke("WriteStartArrayAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartConstructor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartConstructor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartConstructorAsync(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
            .invoke("WriteStartConstructorAsync", (name, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartObjectAsync(
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
        > = __cordl_object.invoke("WriteStartObjectAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenAsync_JsonReader_CancellationToken0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteTokenAsync", (reader, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenAsync_JsonReader__cordl_bool_CancellationToken1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        writeChildren: bool,
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
            .invoke("WriteTokenAsync", (reader, writeChildren, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenAsync_JsonReader__cordl_bool__cordl_bool__cordl_bool_CancellationToken4(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        writeChildren: bool,
        writeDateConstructorAsDate: bool,
        writeComments: bool,
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
                "WriteTokenAsync",
                (
                    reader,
                    writeChildren,
                    writeDateConstructorAsDate,
                    writeComments,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenAsync_JsonToken_CancellationToken2(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteTokenAsync", (token, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenAsync_JsonToken_Il2CppObject_CancellationToken3(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteTokenAsync", (token, value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTokenSyncReadingAsync(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
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
            .invoke("WriteTokenSyncReadingAsync", (reader, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToken_JsonReader0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToken", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToken_JsonReader__cordl_bool1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        writeChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToken", (reader, writeChildren))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToken_JsonReader__cordl_bool__cordl_bool__cordl_bool4(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        writeChildren: bool,
        writeDateConstructorAsDate: bool,
        writeComments: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteToken",
                (reader, writeChildren, writeDateConstructorAsDate, writeComments),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToken_JsonToken3(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToken_JsonToken_Il2CppObject2(
        &mut self,
        token: crate::Newtonsoft::Json::JsonToken,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToken", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUndefined(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUndefined", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUndefinedAsync(
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
        > = __cordl_object.invoke("WriteUndefinedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_DateTimeOffset_CancellationToken9(
        &mut self,
        value: crate::System::DateTimeOffset,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_DateTime_CancellationToken7(
        &mut self,
        value: crate::System::DateTime,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Decimal_CancellationToken11(
        &mut self,
        value: crate::System::Decimal,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Guid_CancellationToken17(
        &mut self,
        value: crate::System::Guid,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Il2CppArray_CancellationToken4(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Il2CppObject_CancellationToken23(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Il2CppString_CancellationToken28(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_JsonWriter_PrimitiveTypeCode_Il2CppObject_CancellationToken38(
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        typeCode: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteValueAsync", (writer, typeCode, value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken1(
        &mut self,
        value: crate::System::Nullable_1<bool>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken10(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTimeOffset>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken12(
        &mut self,
        value: crate::System::Nullable_1<crate::System::Decimal>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken14(
        &mut self,
        value: crate::System::Nullable_1<f64>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken16(
        &mut self,
        value: crate::System::Nullable_1<f32>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken18(
        &mut self,
        value: crate::System::Nullable_1<crate::System::Guid>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken20(
        &mut self,
        value: crate::System::Nullable_1<i32>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken22(
        &mut self,
        value: crate::System::Nullable_1<i64>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken25(
        &mut self,
        value: crate::System::Nullable_1<i8>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken27(
        &mut self,
        value: crate::System::Nullable_1<i16>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken3(
        &mut self,
        value: crate::System::Nullable_1<u8>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken30(
        &mut self,
        value: crate::System::Nullable_1<crate::System::TimeSpan>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken32(
        &mut self,
        value: crate::System::Nullable_1<u32>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken34(
        &mut self,
        value: crate::System::Nullable_1<u64>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken37(
        &mut self,
        value: crate::System::Nullable_1<u16>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken6(
        &mut self,
        value: crate::System::Nullable_1<char>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Nullable_1_CancellationToken8(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTime>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_TimeSpan_CancellationToken29(
        &mut self,
        value: crate::System::TimeSpan,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_Uri_CancellationToken35(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync__cordl_bool_CancellationToken0(
        &mut self,
        value: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync__cordl_char_CancellationToken5(
        &mut self,
        value: char,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_f32_CancellationToken15(
        &mut self,
        value: f32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_f64_CancellationToken13(
        &mut self,
        value: f64,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_i16_CancellationToken26(
        &mut self,
        value: i16,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_i32_CancellationToken19(
        &mut self,
        value: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_i64_CancellationToken21(
        &mut self,
        value: i64,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_i8_CancellationToken24(
        &mut self,
        value: i8,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_u16_CancellationToken36(
        &mut self,
        value: u16,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_u32_CancellationToken31(
        &mut self,
        value: u32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_u64_CancellationToken33(
        &mut self,
        value: u64,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueAsync_u8_CancellationToken2(
        &mut self,
        value: u8,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteValueAsync", (value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueDelimiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValueDelimiter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueDelimiterAsync(
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
        > = __cordl_object.invoke("WriteValueDelimiterAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_DateTime14(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_DateTimeOffset15(
        &mut self,
        value: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Decimal13(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Guid16(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Il2CppArray35(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Il2CppObject37(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Il2CppString0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_JsonWriter_PrimitiveTypeCode_Il2CppObject38(
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        typeCode: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteValue", (writer, typeCode, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_18(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_19(
        &mut self,
        value: crate::System::Nullable_1<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_20(
        &mut self,
        value: crate::System::Nullable_1<i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_21(
        &mut self,
        value: crate::System::Nullable_1<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_22(
        &mut self,
        value: crate::System::Nullable_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_23(
        &mut self,
        value: crate::System::Nullable_1<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_24(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_25(
        &mut self,
        value: crate::System::Nullable_1<i16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_26(
        &mut self,
        value: crate::System::Nullable_1<u16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_27(
        &mut self,
        value: crate::System::Nullable_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_28(
        &mut self,
        value: crate::System::Nullable_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_29(
        &mut self,
        value: crate::System::Nullable_1<i8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_30(
        &mut self,
        value: crate::System::Nullable_1<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_31(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_32(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_33(
        &mut self,
        value: crate::System::Nullable_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Nullable_1_34(
        &mut self,
        value: crate::System::Nullable_1<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_TimeSpan17(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_Uri36(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue__cordl_bool7(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue__cordl_char10(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_f32_5(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_f64_6(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_i16_8(
        &mut self,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_i32_1(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_i64_3(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_i8_12(
        &mut self,
        value: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_u16_9(
        &mut self,
        value: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_u32_2(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_u64_4(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue_u8_11(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWhitespace(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteWhitespace", (ws))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWhitespaceAsync(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteWhitespaceAsync", (ws, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _InternalWriteEndAsync_g__AwaitEnd_11_2(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        LevelsToComplete: i32,
        CancellationToken: crate::System::Threading::CancellationToken,
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
                "<InternalWriteEndAsync>g__AwaitEnd|11_2",
                (task, LevelsToComplete, CancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _InternalWriteEndAsync_g__AwaitIndent_11_1(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        LevelsToComplete: i32,
        token: crate::Newtonsoft::Json::JsonToken,
        CancellationToken: crate::System::Threading::CancellationToken,
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
                "<InternalWriteEndAsync>g__AwaitIndent|11_1",
                (task, LevelsToComplete, token, CancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _InternalWriteEndAsync_g__AwaitProperty_11_0(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        LevelsToComplete: i32,
        token: crate::Newtonsoft::Json::JsonToken,
        CancellationToken: crate::System::Threading::CancellationToken,
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
                "<InternalWriteEndAsync>g__AwaitProperty|11_0",
                (task, LevelsToComplete, token, CancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _InternalWriteEndAsync_g__AwaitRemaining_11_3(
        &mut self,
        LevelsToComplete: i32,
        CancellationToken: crate::System::Threading::CancellationToken,
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
                "<InternalWriteEndAsync>g__AwaitRemaining|11_3",
                (LevelsToComplete, CancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AutoCompleteOnClose(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AutoCompleteOnClose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CloseOutput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CloseOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContainerPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ContainerPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Culture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("get_Culture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateFormatHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateFormatHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateFormatHandling = __cordl_object
            .invoke("get_DateFormatHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateFormatString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DateFormatString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateTimeZoneHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateTimeZoneHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateTimeZoneHandling = __cordl_object
            .invoke("get_DateTimeZoneHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FloatFormatHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::FloatFormatHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::FloatFormatHandling = __cordl_object
            .invoke("get_FloatFormatHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Formatting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Formatting> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Formatting = __cordl_object
            .invoke("get_Formatting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Path(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Path", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StringEscapeHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::StringEscapeHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::StringEscapeHandling = __cordl_object
            .invoke("get_StringEscapeHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Top(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Top", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::WriteState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::WriteState = __cordl_object
            .invoke("get_WriteState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AutoCompleteOnClose(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AutoCompleteOnClose", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CloseOutput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CloseOutput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Culture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Culture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DateFormatHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateFormatHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DateFormatString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateFormatString", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_FloatFormatHandling(
        &mut self,
        value: crate::Newtonsoft::Json::FloatFormatHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FloatFormatHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Formatting(
        &mut self,
        value: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Formatting", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StringEscapeHandling(
        &mut self,
        value: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StringEscapeHandling", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl AsRef<crate::System::IDisposable> for crate::Newtonsoft::Json::JsonWriter {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter")]
impl AsMut<crate::System::IDisposable> for crate::Newtonsoft::Json::JsonWriter {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonWriter_State {
    #[default]
    Array = 5i32,
    ArrayStart = 4i32,
    Closed = 8i32,
    Constructor = 7i32,
    ConstructorStart = 6i32,
    Error = 9i32,
    Object = 3i32,
    ObjectStart = 2i32,
    Property = 1i32,
    Start = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::JsonWriter_State {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter/State";
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::JsonWriter_State {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::JsonWriter_State {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::JsonWriter_State {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+State")]
unsafe impl quest_hook::libil2cpp::Return for crate::Newtonsoft::Json::JsonWriter_State {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub __4__this: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
    pub LevelsToComplete: i32,
    pub CancellationToken: crate::System::Threading::CancellationToken,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter/<<InternalWriteEndAsync>g__AwaitEnd|11_2>d";
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
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
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
impl crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitEnd_11_2_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitEnd_11_2_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub __4__this: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
    pub token: crate::Newtonsoft::Json::JsonToken,
    pub CancellationToken: crate::System::Threading::CancellationToken,
    pub LevelsToComplete: i32,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter/<<InternalWriteEndAsync>g__AwaitIndent|11_1>d";
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
impl crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitIndent_11_1_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitIndent_11_1_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub __4__this: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
    pub CancellationToken: crate::System::Threading::CancellationToken,
    pub token: crate::Newtonsoft::Json::JsonToken,
    pub LevelsToComplete: i32,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter/<<InternalWriteEndAsync>g__AwaitProperty|11_0>d";
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
impl crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitProperty_11_0_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitProperty_11_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
    pub CancellationToken: crate::System::Threading::CancellationToken,
    pub LevelsToComplete: i32,
    pub _token_5__2: crate::Newtonsoft::Json::JsonToken,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json";
    const CLASS_NAME: &'static str = "JsonWriter/<<InternalWriteEndAsync>g__AwaitRemaining|11_3>d";
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
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
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
impl crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "Newtonsoft+Json+JsonWriter+__InternalWriteEndAsync_g__AwaitRemaining_11_3_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::Newtonsoft::Json::JsonWriter___InternalWriteEndAsync_g__AwaitRemaining_11_3_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
