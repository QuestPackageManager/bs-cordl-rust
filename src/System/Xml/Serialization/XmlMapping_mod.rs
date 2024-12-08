#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlMapping {
    __cordl_parent: crate::System::Object,
    pub map: *mut crate::System::Xml::Serialization::ObjectMap,
    pub relatedMaps: *mut crate::System::Collections::ArrayList,
    pub format: crate::System::Xml::Serialization::SerializationFormat,
    pub source: *mut crate::System::Xml::Serialization::SerializationSource,
    pub _elementName: *mut crate::System::String,
    pub _namespace: *mut crate::System::String,
    pub key: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlMapping =>
    "System.Xml.Serialization"."XmlMapping"
);
#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlMapping {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
impl crate::System::Xml::Serialization::XmlMapping {
    pub fn New(
        elementName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementName, ns))?;
        Ok(__cordl_object)
    }
    pub fn SetKey(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        elementName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ElementName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Serialization::SerializationFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Serialization::SerializationFormat = __cordl_object
            .invoke("get_Format", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Namespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::ObjectMap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::ObjectMap = __cordl_object
            .invoke("get_ObjectMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RelatedMaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_RelatedMaps", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Source(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::SerializationSource,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::SerializationSource = __cordl_object
            .invoke("get_Source", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Format(
        &mut self,
        value: crate::System::Xml::Serialization::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Format", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ObjectMap(
        &mut self,
        value: *mut crate::System::Xml::Serialization::ObjectMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectMap", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RelatedMaps(
        &mut self,
        value: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RelatedMaps", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlMapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
