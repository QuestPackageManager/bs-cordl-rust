#[cfg(feature = "System+Xml+XPath+XPathDocument")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathDocument {
    __cordl_parent: crate::System::Object,
    pub pageXmlNmsp: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub idxXmlNmsp: i32,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub hasLineInfo: bool,
    pub mapNmsp: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::MS::Internal::Xml::Cache::XPathNodeRef,
        crate::MS::Internal::Xml::Cache::XPathNodeRef,
    >,
}
#[cfg(feature = "System+Xml+XPath+XPathDocument")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathDocument =>
    "System.Xml.XPath"."XPathDocument"
);
#[cfg(feature = "System+Xml+XPath+XPathDocument")]
impl std::ops::Deref for crate::System::Xml::XPath::XPathDocument {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathDocument")]
impl std::ops::DerefMut for crate::System::Xml::XPath::XPathDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathDocument")]
impl crate::System::Xml::XPath::XPathDocument {
    pub fn get_HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasLineInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespaces(
        &mut self,
        pageElem: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idxElem: i32,
        pageNmsp: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LookupNamespaces", (pageElem, idxElem, pageNmsp))?;
        Ok(__cordl_ret)
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetXmlNamespaceNode(
        &mut self,
        pageXmlNmsp: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetXmlNamespaceNode", (pageXmlNmsp))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XPath+XPathDocument")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XPath::XPathDocument {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
