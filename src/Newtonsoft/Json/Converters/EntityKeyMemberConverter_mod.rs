#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct EntityKeyMemberConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
}
#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::EntityKeyMemberConverter =>
    "Newtonsoft.Json.Converters"."EntityKeyMemberConverter"
);
#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::EntityKeyMemberConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Converters::EntityKeyMemberConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
impl crate::Newtonsoft::Json::Converters::EntityKeyMemberConverter {
    pub const EntityKeyMemberFullTypeName: &'static str = "System.Data.EntityKeyMember";
    pub const KeyPropertyName: &'static str = "Key";
    pub const TypePropertyName: &'static str = "Type";
    pub const ValuePropertyName: &'static str = "Value";
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
    pub fn ReadJson(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn CanConvert(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn WriteJson(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+EntityKeyMemberConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::EntityKeyMemberConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
