#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonArray {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonToken,
    pub _children: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Bson::BsonToken,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonArray =>
    "Newtonsoft.Json.Bson"."BsonArray"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonArray {
    type Target = crate::Newtonsoft::Json::Bson::BsonToken;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
impl crate::Newtonsoft::Json::Bson::BsonArray {
    pub fn Add(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Bson::BsonToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::Newtonsoft::Json::Bson::BsonToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::Newtonsoft::Json::Bson::BsonToken,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonArray")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}