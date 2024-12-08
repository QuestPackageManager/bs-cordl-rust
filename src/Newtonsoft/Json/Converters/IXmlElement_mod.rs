#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
#[repr(C)]
#[derive(Debug)]
pub struct IXmlElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::IXmlElement =>
    "Newtonsoft.Json.Converters"."IXmlElement"
);
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::IXmlElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl crate::Newtonsoft::Json::Converters::IXmlElement {
    pub fn GetPrefixOfNamespace(
        &mut self,
        namespaceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPrefixOfNamespace", (namespaceUri))?;
        Ok(__cordl_ret)
    }
    pub fn SetAttributeNode(
        &mut self,
        attribute: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributeNode", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+IXmlElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::IXmlElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}