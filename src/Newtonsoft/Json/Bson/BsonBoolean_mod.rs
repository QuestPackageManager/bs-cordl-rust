#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonBoolean {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonValue>,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonBoolean =>
    "Newtonsoft.Json.Bson"."BsonBoolean"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonBoolean {
    type Target = quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonBoolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
impl crate::Newtonsoft::Json::Bson::BsonBoolean {
    pub fn New(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBoolean")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonBoolean {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
