#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathDocumentNavigator {
    __cordl_parent: crate::System::Xml::XPath::XPathNavigator,
    pub _pageCurrent: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub _pageParent: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub _idxCurrent: i32,
    pub _idxParent: i32,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathDocumentNavigator
    => "MS.Internal.Xml.Cache"."XPathDocumentNavigator"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathDocumentNavigator {
    type Target = crate::System::Xml::XPath::XPathNavigator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::Cache::XPathDocumentNavigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
impl crate::MS::Internal::Xml::Cache::XPathDocumentNavigator {
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
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
    pub fn MoveToNextNamespace(
        &mut self,
        scope: crate::System::Xml::XPath::XPathNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextNamespace", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XPath::XPathNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPositionHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPositionHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToParent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NamespaceURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToFirstNamespace(
        &mut self,
        namespaceScope: crate::System::Xml::XPath::XPathNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToFirstNamespace", (namespaceScope))?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
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
    pub fn _ctor(
        &mut self,
        pageCurrent: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idxCurrent: i32,
        pageParent: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idxParent: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pageCurrent, idxCurrent, pageParent, idxParent))?;
        Ok(__cordl_ret)
    }
    pub fn IsSamePosition(
        &mut self,
        other: *mut crate::System::Xml::XPath::XPathNavigator,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSamePosition", (other))?;
        Ok(__cordl_ret)
    }
    pub fn get_UnderlyingObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_UnderlyingObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XPath::XPathNavigator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XPath::XPathNavigator = __cordl_object
            .invoke("Clone", ())?;
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
    pub fn New(
        pageCurrent: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idxCurrent: i32,
        pageParent: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idxParent: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pageCurrent, idxCurrent, pageParent, idxParent))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathDocumentNavigator")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::Cache::XPathDocumentNavigator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
