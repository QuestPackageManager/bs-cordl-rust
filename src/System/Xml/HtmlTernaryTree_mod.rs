#[cfg(feature = "System+Xml+HtmlTernaryTree")]
#[repr(C)]
#[derive(Debug)]
pub struct HtmlTernaryTree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+HtmlTernaryTree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::HtmlTernaryTree => "System.Xml"
    ."HtmlTernaryTree"
);
#[cfg(feature = "System+Xml+HtmlTernaryTree")]
impl std::ops::Deref for crate::System::Xml::HtmlTernaryTree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlTernaryTree")]
impl std::ops::DerefMut for crate::System::Xml::HtmlTernaryTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlTernaryTree")]
impl crate::System::Xml::HtmlTernaryTree {}
#[cfg(feature = "System+Xml+HtmlTernaryTree")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::HtmlTernaryTree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
