#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlElementWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XmlNodeWrapper,
    pub _element: *mut crate::System::Xml::XmlElement,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XmlElementWrapper
    => "Newtonsoft.Json.Converters"."XmlElementWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XmlNodeWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl crate::Newtonsoft::Json::Converters::XmlElementWrapper {
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
    pub fn New(
        element: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (element))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        element: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (element))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
