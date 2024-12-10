#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct XElementWrapper {
    __cordl_parent: crate::Newtonsoft::Json::Converters::XContainerWrapper,
    pub _attributes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Newtonsoft::Json::Converters::IXmlNode,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Converters::XElementWrapper =>
    "Newtonsoft.Json.Converters"."XElementWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::XElementWrapper {
    type Target = crate::Newtonsoft::Json::Converters::XContainerWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl crate::Newtonsoft::Json::Converters::XElementWrapper {
    pub fn AppendChild(
        &mut self,
        newChild: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Converters::IXmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Converters::IXmlNode,
        > = __cordl_object.invoke("AppendChild", (newChild))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn HasImplicitNamespaceAttribute(
        &mut self,
        namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasImplicitNamespaceAttribute", (namespaceUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
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
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::IXmlNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Newtonsoft::Json::Converters::IXmlNode,
            >,
        > = __cordl_object.invoke("get_Attributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Element(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement> = __cordl_object
            .invoke("get_Element", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmpty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_NamespaceUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl AsRef<crate::Newtonsoft::Json::Converters::IXmlElement>
for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Converters::IXmlElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl AsMut<crate::Newtonsoft::Json::Converters::IXmlElement>
for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Converters::IXmlElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl AsRef<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+XElementWrapper")]
impl AsMut<crate::Newtonsoft::Json::Converters::IXmlNode>
for crate::Newtonsoft::Json::Converters::XElementWrapper {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Converters::IXmlNode {
        unsafe { std::mem::transmute(self) }
    }
}
