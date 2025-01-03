#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
#[repr(C)]
#[derive(Debug)]
pub struct XStreamingElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XName>,
    pub content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XStreamingElement =>
    "System.Xml.Linq"."XStreamingElement"
);
#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
impl std::ops::Deref for crate::System::Xml::Linq::XStreamingElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XStreamingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
impl crate::System::Xml::Linq::XStreamingElement {}
#[cfg(feature = "System+Xml+Linq+XStreamingElement")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::XStreamingElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
