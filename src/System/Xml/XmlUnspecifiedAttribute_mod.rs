#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlUnspecifiedAttribute {
    __cordl_parent: crate::System::Xml::XmlAttribute,
    pub fSpecified: bool,
}
#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlUnspecifiedAttribute =>
    "System.Xml"."XmlUnspecifiedAttribute"
);
#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
impl std::ops::Deref for crate::System::Xml::XmlUnspecifiedAttribute {
    type Target = crate::System::Xml::XmlAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
impl std::ops::DerefMut for crate::System::Xml::XmlUnspecifiedAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
impl crate::System::Xml::XmlUnspecifiedAttribute {
    pub fn AppendChild(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("AppendChild", (newChild))?;
        Ok(__cordl_ret)
    }
    pub fn CloneNode(
        &mut self,
        deep: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("CloneNode", (deep))?;
        Ok(__cordl_ret)
    }
    pub fn InsertAfter(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
        refChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("InsertAfter", (newChild, refChild))?;
        Ok(__cordl_ret)
    }
    pub fn InsertBefore(
        &mut self,
        newChild: *mut crate::System::Xml::XmlNode,
        refChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("InsertBefore", (newChild, refChild))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
        doc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefix, localName, namespaceURI, doc))?;
        Ok(__cordl_object)
    }
    pub fn RemoveChild(
        &mut self,
        oldChild: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("RemoveChild", (oldChild))?;
        Ok(__cordl_ret)
    }
    pub fn SetSpecified(
        &mut self,
        f: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSpecified", (f))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTo(
        &mut self,
        w: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
        doc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefix, localName, namespaceURI, doc))?;
        Ok(__cordl_ret)
    }
    pub fn get_Specified(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Specified", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InnerText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InnerText", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlUnspecifiedAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlUnspecifiedAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
