#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonString {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonValue,
    pub _ByteCount_k__BackingField: i32,
    pub _IncludeLength_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonString =>
    "Newtonsoft.Json.Bson"."BsonString"
);
#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonString {
    type Target = crate::Newtonsoft::Json::Bson::BsonValue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
impl crate::Newtonsoft::Json::Bson::BsonString {
    pub fn get_ByteCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ByteCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        value: *mut crate::System::Object,
        includeLength: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value, includeLength))?;
        Ok(__cordl_ret)
    }
    pub fn get_IncludeLength(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IncludeLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ByteCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ByteCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        value: *mut crate::System::Object,
        includeLength: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, includeLength))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonString")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
