#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonFormatterConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _reader: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
    >,
    pub _contract: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
    >,
    pub _member: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonProperty,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonFormatterConverter =>
    "Newtonsoft.Json.Serialization"."JsonFormatterConverter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Convert_Type0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Convert", (value, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Convert_TypeCode1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        typeCode: crate::System::TypeCode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Convert", (value, typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTokenValue<T>(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetTokenValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, contract, member))?;
        Ok(__cordl_object.into())
    }
    pub fn ToBoolean(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBoolean", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToChar(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ToChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDecimal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ToDouble", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ToInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i8 = __cordl_object.invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ToSingle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("ToUInt32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("ToUInt64", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, contract, member))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl AsRef<crate::System::Runtime::Serialization::IFormatterConverter>
for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::IFormatterConverter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonFormatterConverter")]
impl AsMut<crate::System::Runtime::Serialization::IFormatterConverter>
for crate::Newtonsoft::Json::Serialization::JsonFormatterConverter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IFormatterConverter {
        unsafe { std::mem::transmute(self) }
    }
}
