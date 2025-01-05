#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonRegex {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonToken,
    pub _Pattern_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Bson::BsonString,
    >,
    pub _Options_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Bson::BsonString,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonRegex =>
    "Newtonsoft.Json.Bson"."BsonRegex"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonRegex {
    type Target = crate::Newtonsoft::Json::Bson::BsonToken;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonRegex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
impl crate::Newtonsoft::Json::Bson::BsonRegex {
    pub fn New(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pattern, options))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pattern, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Bson::BsonString,
        > = __cordl_object.invoke("get_Options", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Bson::BsonString,
        > = __cordl_object.invoke("get_Pattern", ())?;
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
    pub fn set_Options(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Options", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Pattern(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Pattern", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonRegex")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonRegex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
