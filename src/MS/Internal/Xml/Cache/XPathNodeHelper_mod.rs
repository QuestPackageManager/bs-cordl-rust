#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathNodeHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::Cache::XPathNodeHelper =>
    "MS.Internal.Xml.Cache"."XPathNodeHelper"
);
#[cfg(feature = "MS+Internal+Xml+Cache+XPathNodeHelper")]
impl std::ops::Deref for crate::MS::Internal::Xml::Cache::XPathNodeHelper {
    type Target = crate::System::Object;
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
impl crate::MS::Internal::Xml::Cache::XPathNodeHelper {}
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
