#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonBinary {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonValue,
    pub _BinaryType_k__BackingField: crate::Newtonsoft::Json::Bson::BsonBinaryType,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonBinary =>
    "Newtonsoft.Json.Bson"."BsonBinary"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonBinary {
    type Target = crate::Newtonsoft::Json::Bson::BsonValue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonBinary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl crate::Newtonsoft::Json::Bson::BsonBinary {
    pub fn set_BinaryType(
        &mut self,
        value: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BinaryType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        binaryType: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value, binaryType))?;
        Ok(__cordl_ret)
    }
    pub fn get_BinaryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonBinaryType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonBinaryType = __cordl_object
            .invoke("get_BinaryType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        binaryType: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, binaryType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonBinary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
