#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeWrapper {
    __cordl_parent: crate::System::Object,
    pub _node: *mut crate::System::Xml::XmlNode,
    pub _childNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    >,
    pub _attributes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XmlNodeWrapper =>
    "Newtonsoft.Json.Converters"."XmlNodeWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XmlNodeWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XmlNodeWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
impl crate::Newtonsoft::Json::Converters::XmlNodeWrapper {
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Value(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AppendChild(
        &mut self,
        newChild: *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("AppendChild", (newChild))?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NamespaceUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("get_ChildNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WrappedNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_WrappedNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Converters::IXmlNode = __cordl_object
            .invoke("get_ParentNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasChildNodes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasChildNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasAttributes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("get_Attributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XmlNodeWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XmlNodeWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
