#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathNodeHelper =>
    "MS.Internal.Xml.Cache"."XPathNodeHelper"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
impl crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    pub fn GetInScopeNamespaces(
        pageElem: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxElem: i32,
        pageNmsp: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInScopeNamespaces", (pageElem, idxElem, pageNmsp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalNamespaces(
        pageElem: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxElem: i32,
        pageNmsp: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalNamespaces", (pageElem, idxElem, pageNmsp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocation(
        pageNode: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxNode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocation", (pageNode, idxNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonDescendant(
        pageNode: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxNode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonDescendant", (pageNode, idxNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParent(
        pageNode: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxNode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParent", (pageNode, idxNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextFollowing(
        pageCurrent: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxCurrent: quest_hook::libil2cpp::ByRefMut<i32>,
        pageEnd: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::MS::Internal::Xml::Cache::XPathNode,
            >,
        >,
        idxEnd: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTextFollowing", (pageCurrent, idxCurrent, pageEnd, idxEnd))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
