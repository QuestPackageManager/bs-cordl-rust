#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonBinaryWriter {
    __cordl_parent: crate::System::Object,
    pub _writer: *mut crate::System::IO::BinaryWriter,
    pub _largeByteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _DateTimeKindHandling_k__BackingField: crate::System::DateTimeKind,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonBinaryWriter =>
    "Newtonsoft.Json.Bson"."BsonBinaryWriter"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonBinaryWriter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonBinaryWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
impl crate::Newtonsoft::Json::Bson::BsonBinaryWriter {
    pub fn CalculateSizeWithLength(
        &mut self,
        stringByteCount: i32,
        includeSize: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateSizeWithLength", (stringByteCount, includeSize))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSize_BsonToken1(
        &mut self,
        t: *mut crate::Newtonsoft::Json::Bson::BsonToken,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CalculateSize", (t))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateSize_i32_0(
        &mut self,
        stringByteCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateSize", (stringByteCount))?;
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
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        writer: *mut crate::System::IO::BinaryWriter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer))?;
        Ok(__cordl_object)
    }
    pub fn WriteString(
        &mut self,
        s: *mut crate::System::String,
        byteCount: i32,
        calculatedlengthPrefix: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (s, byteCount, calculatedlengthPrefix))?;
        Ok(__cordl_ret)
    }
    pub fn WriteToken(
        &mut self,
        t: *mut crate::Newtonsoft::Json::Bson::BsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToken", (t))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTokenInternal(
        &mut self,
        t: *mut crate::Newtonsoft::Json::Bson::BsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTokenInternal", (t))?;
        Ok(__cordl_ret)
    }
    pub fn WriteUtf8Bytes(
        &mut self,
        s: *mut crate::System::String,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUtf8Bytes", (s, byteCount))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        writer: *mut crate::System::IO::BinaryWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer))?;
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
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Bson::BsonBinaryWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
