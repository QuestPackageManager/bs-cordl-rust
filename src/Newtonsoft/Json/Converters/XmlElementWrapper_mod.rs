#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlElementWrapper {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XmlNodeWrapper,
    >,
    pub _element: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XmlElementWrapper
    => "Newtonsoft.Json.Converters"."XmlElementWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::XmlNodeWrapper,
    >;
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
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPrefixOfNamespace", (namespaceUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (element))?;
        Ok(__cordl_object.into())
    }
    pub fn SetAttributeNode(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributeNode", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>>
for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlElement>>
for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Converters::IXmlElement,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlElementWrapper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>>
for crate::Newtonsoft::Json::Converters::XmlElementWrapper {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode> {
        unsafe { std::mem::transmute(self) }
    }
}
