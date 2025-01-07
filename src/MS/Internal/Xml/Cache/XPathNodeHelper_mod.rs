#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.Cache";
    const CLASS_NAME: &'static str = "XPathNodeHelper";
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
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
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::MS::Internal::Xml::Cache::XPathNode,
                >,
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
