#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodeInfoAtom {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _namespaceUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _pageParent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::MS::Internal::Xml::Cache::XPathNode>,
    >,
    pub _pageSibling: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::MS::Internal::Xml::Cache::XPathNode>,
    >,
    pub _doc: quest_hook::libil2cpp::Gc<crate::System::Xml::XPath::XPathDocument>,
    pub _lineNumBase: i32,
    pub _linePosBase: i32,
    pub _pageInfo: quest_hook::libil2cpp::Gc<
        crate::MS::Internal::Xml::Cache::XPathNodePageInfo,
    >,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.Cache";
    const CLASS_NAME: &'static str = "XPathNodeInfoAtom";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeInfoAtom")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathNodeInfoAtom {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XPath::XPathDocument>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XPath::XPathDocument,
        > = __cordl_object.invoke("get_Document", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumberBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumberBase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePositionBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePositionBase", ())?;
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
    pub fn get_PageInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::Cache::XPathNodePageInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::Cache::XPathNodePageInfo,
        > = __cordl_object.invoke("get_PageInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        > = __cordl_object.invoke("get_ParentPage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Prefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SiblingPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        > = __cordl_object.invoke("get_SiblingPage", ())?;
        Ok(__cordl_ret.into())
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
