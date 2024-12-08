#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodeInfoAtom {
    __cordl_parent: crate::System::Object,
    pub _localName: *mut crate::System::String,
    pub _namespaceUri: *mut crate::System::String,
    pub _prefix: *mut crate::System::String,
    pub _pageParent: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub _pageSibling: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub _doc: *mut crate::System::Xml::XPath::XPathDocument,
    pub _lineNumBase: i32,
    pub _linePosBase: i32,
    pub _pageInfo: *mut crate::MS::Internal::Xml::Cache::XPathNodePageInfo,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathNodeInfoAtom =>
    "MS.Internal.Xml.Cache"."XPathNodeInfoAtom"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
impl crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XPath::XPathDocument> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XPath::XPathDocument = __cordl_object
            .invoke("get_Document", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNumberBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumberBase", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePositionBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePositionBase", ())?;
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
    pub fn get_PageInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::MS::Internal::Xml::Cache::XPathNodePageInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::Cache::XPathNodePageInfo = __cordl_object
            .invoke("get_PageInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        > = __cordl_object.invoke("get_ParentPage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Prefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SiblingPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        > = __cordl_object.invoke("get_SiblingPage", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}