#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonFormatterConverter {
    __cordl_parent: crate::System::Object,
    pub _reader: *mut crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
    pub _contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
    pub _member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonFormatterConverter =>
    "Newtonsoft.Json.Serialization"."JsonFormatterConverter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    pub fn ToUInt32(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("ToUInt32", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt64(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ToInt64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDecimal(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDouble(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ToDouble", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToSingle(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ToSingle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt32(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToInt32", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToBoolean(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBoolean", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToUInt64(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("ToUInt64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt16(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("ToInt16", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, contract, member))?;
        Ok(__cordl_ret)
    }
    pub fn GetTokenValue<T>(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetTokenValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToSByte(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i8 = __cordl_object.invoke("ToSByte", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToByte(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ToByte", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToChar(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ToChar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Convert_Type0(
        &mut self,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Convert", (value, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Convert_TypeCode1(
        &mut self,
        value: *mut crate::System::Object,
        typeCode: crate::System::TypeCode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Convert", (value, typeCode))?;
        Ok(__cordl_ret)
    }
    pub fn ToUInt16(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("ToUInt16", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, contract, member))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
