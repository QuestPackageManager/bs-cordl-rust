#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodePageInfo {
    __cordl_parent: crate::System::Object,
    pub _pageNum: i32,
    pub _nodeCount: i32,
    pub _pageNext: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathNodePageInfo =>
    "MS.Internal.Xml.Cache"."XPathNodePageInfo"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathNodePageInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::Cache::XPathNodePageInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
impl crate::MS::Internal::Xml::Cache::XPathNodePageInfo {
    pub fn get_NodeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NodeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NextPage(
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
        > = __cordl_object.invoke("get_NextPage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PageNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PageNumber", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodePageInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::Cache::XPathNodePageInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
