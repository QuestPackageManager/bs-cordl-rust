#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XPathNodeRef {
    pub _page: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::MS::Internal::Xml::Cache::XPathNode,
    >,
    pub _idx: i32,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathNodeRef =>
    "MS.Internal.Xml.Cache"."XPathNodeRef"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::MS::Internal::Xml::Cache::XPathNodeRef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeRef")]
impl crate::MS::Internal::Xml::Cache::XPathNodeRef {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        page: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (page, idx),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Index",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Page(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        >,
    > {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::MS::Internal::Xml::Cache::XPathNode,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Page", ())?;
        Ok(__cordl_ret)
    }
}
