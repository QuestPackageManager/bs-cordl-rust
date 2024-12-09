#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonPropertyCollection {
    __cordl_parent: crate::System::Collections::ObjectModel::KeyedCollection_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    >,
    pub _type: *mut crate::System::Type,
    pub _list: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonPropertyCollection =>
    "Newtonsoft.Json.Serialization"."JsonPropertyCollection"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonPropertyCollection {
    type Target = crate::System::Collections::ObjectModel::KeyedCollection_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonPropertyCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
impl crate::Newtonsoft::Json::Serialization::JsonPropertyCollection {
    pub fn AddProperty(
        &mut self,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddProperty", (property))?;
        Ok(__cordl_ret)
    }
    pub fn GetClosestMatchProperty(
        &mut self,
        propertyName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonProperty = __cordl_object
            .invoke("GetClosestMatchProperty", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyForItem(
        &mut self,
        item: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetKeyForItem", (item))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperty(
        &mut self,
        propertyName: *mut quest_hook::libil2cpp::Il2CppString,
        comparisonType: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonProperty = __cordl_object
            .invoke("GetProperty", (propertyName, comparisonType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn TryGetProperty(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        item: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetProperty", (key, item))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPropertyCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonPropertyCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
