#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceJsonReader {
    __cordl_parent: crate::Newtonsoft::Json::JsonReader,
    pub _innerReader: *mut crate::Newtonsoft::Json::JsonReader,
    pub _textWriter: *mut crate::Newtonsoft::Json::JsonTextWriter,
    pub _sw: *mut crate::System::IO::StringWriter,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Serialization::TraceJsonReader
    => "Newtonsoft.Json.Serialization"."TraceJsonReader"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::TraceJsonReader {
    type Target = crate::Newtonsoft::Json::JsonReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::TraceJsonReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl crate::Newtonsoft::Json::Serialization::TraceJsonReader {
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
    pub fn GetDeserializedJsonMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetDeserializedJsonMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        innerReader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerReader))?;
        Ok(__cordl_object.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_HasLineInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LineNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Newtonsoft_Json_IJsonLineInfo_get_LinePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Newtonsoft.Json.IJsonLineInfo.get_LinePosition", ())?;
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
    pub fn WriteCurrentToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCurrentToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        innerReader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerReader))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
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
    pub fn get_QuoteChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_QuoteChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TokenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::JsonToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::JsonToken = __cordl_object
            .invoke("get_TokenType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::TraceJsonReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl AsRef<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::Serialization::TraceJsonReader {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+TraceJsonReader")]
impl AsMut<crate::Newtonsoft::Json::IJsonLineInfo>
for crate::Newtonsoft::Json::Serialization::TraceJsonReader {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::IJsonLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
