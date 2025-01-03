#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonProperty {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Name_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Bson::BsonString,
    >,
    pub _Value_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Bson::BsonToken,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonProperty =>
    "Newtonsoft.Json.Bson"."BsonProperty"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonProperty {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
impl crate::Newtonsoft::Json::Bson::BsonProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Bson::BsonString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Bson::BsonToken,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonProperty")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
