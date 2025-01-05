#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonEmpty {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>,
    pub _Type_k__BackingField: crate::Newtonsoft::Json::Bson::BsonType,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonEmpty =>
    "Newtonsoft.Json.Bson"."BsonEmpty"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonEmpty {
    type Target = quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonEmpty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
impl crate::Newtonsoft::Json::Bson::BsonEmpty {
    pub fn New(
        _cordl_type: crate::Newtonsoft::Json::Bson::BsonType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonEmpty")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonEmpty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
