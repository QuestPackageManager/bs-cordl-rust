#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonReader {
    __cordl_parent: crate::Newtonsoft::Json::JsonReader,
    pub _reader: *mut crate::System::IO::BinaryReader,
    pub _stack: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext,
    >,
    pub _byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _charBuffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _currentElementType: crate::Newtonsoft::Json::Bson::BsonType,
    pub _bsonReaderState: crate::Newtonsoft::Json::Bson::BsonReader_BsonReaderState,
    pub _currentContext: *mut crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext,
    pub _readRootValueAsArray: bool,
    pub _jsonNet35BinaryCompatibility: bool,
    pub _dateTimeKindHandling: crate::System::DateTimeKind,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonReader =>
    "Newtonsoft.Json.Bson"."BsonReader"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonReader {
    type Target = crate::Newtonsoft::Json::JsonReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
impl crate::Newtonsoft::Json::Bson::BsonReader {
    pub const MaxCharBytesSize: i32 = 128i32;
    #[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+BsonReaderState")]
    pub type BsonReaderState = crate::Newtonsoft::Json::Bson::BsonReader_BsonReaderState;
    #[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
    pub type ContainerContext = crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext;
    pub fn BytesInSequence(&mut self, b: u8) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("BytesInSequence", (b))?;
        Ok(__cordl_ret)
    }
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
    pub fn EnsureBuffers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureBuffers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLastFullCharStop(
        &mut self,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLastFullCharStop", (start))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetString", (length))?;
        Ok(__cordl_ret)
    }
    pub fn MovePosition(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePosition", (count))?;
        Ok(__cordl_ret)
    }
    pub fn New_BinaryReader1(
        reader: *mut crate::System::IO::BinaryReader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object)
    }
    pub fn New_BinaryReader__cordl_bool_DateTimeKind3(
        reader: *mut crate::System::IO::BinaryReader,
        readRootValueAsArray: bool,
        dateTimeKindHandling: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, readRootValueAsArray, dateTimeKindHandling))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream0(
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream__cordl_bool_DateTimeKind2(
        stream: *mut crate::System::IO::Stream,
        readRootValueAsArray: bool,
        dateTimeKindHandling: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, readRootValueAsArray, dateTimeKindHandling))?;
        Ok(__cordl_object)
    }
    pub fn PopContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushContext(
        &mut self,
        newContext: *mut crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushContext", (newContext))?;
        Ok(__cordl_ret)
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBinary(
        &mut self,
        binaryType: quest_hook::libil2cpp::ByRefMut<
            crate::Newtonsoft::Json::Bson::BsonBinaryType,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadBinary", (binaryType))?;
        Ok(__cordl_ret)
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadBytes(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadBytes", (count))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCodeWScope(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadCodeWScope", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ReadDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ReadElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt32(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadInt32", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt64(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ReadInt64", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadLengthString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ReadLengthString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadNormal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadNormal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadReference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ReadString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadType_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonType = __cordl_object
            .invoke("ReadType", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadType_BsonType0(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::Bson::BsonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BinaryReader1(
        &mut self,
        reader: *mut crate::System::IO::BinaryReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BinaryReader__cordl_bool_DateTimeKind3(
        &mut self,
        reader: *mut crate::System::IO::BinaryReader,
        readRootValueAsArray: bool,
        dateTimeKindHandling: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, readRootValueAsArray, dateTimeKindHandling))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream0(
        &mut self,
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream__cordl_bool_DateTimeKind2(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        readRootValueAsArray: bool,
        dateTimeKindHandling: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, readRootValueAsArray, dateTimeKindHandling))?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTimeKindHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeKind> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTimeKind = __cordl_object
            .invoke("get_DateTimeKindHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_JsonNet35BinaryCompatibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_JsonNet35BinaryCompatibility", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadRootValueAsArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ReadRootValueAsArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DateTimeKindHandling(
        &mut self,
        value: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateTimeKindHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_JsonNet35BinaryCompatibility(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_JsonNet35BinaryCompatibility", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReadRootValueAsArray(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadRootValueAsArray", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+BsonReaderState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsonReader_BsonReaderState {
    CodeWScopeCode = 5i32,
    CodeWScopeScope = 6i32,
    CodeWScopeScopeEnd = 8i32,
    CodeWScopeScopeObject = 7i32,
    CodeWScopeStart = 4i32,
    Normal = 0i32,
    ReferenceId = 3i32,
    ReferenceRef = 2i32,
    ReferenceStart = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+BsonReaderState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Bson::BsonReader_BsonReaderState => "Newtonsoft.Json.Bson"
    ."BsonReader/BsonReaderState"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonReader_ContainerContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: crate::Newtonsoft::Json::Bson::BsonType,
    pub Length: i32,
    pub Position: i32,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Bson::BsonReader_ContainerContext => "Newtonsoft.Json.Bson"
    ."BsonReader/ContainerContext"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
impl crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext {
    pub fn New(
        _cordl_type: crate::Newtonsoft::Json::Bson::BsonType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::Bson::BsonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonReader+ContainerContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Bson::BsonReader_ContainerContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
