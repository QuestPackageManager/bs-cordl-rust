#[cfg(feature = "System+Xml+Ref")]
#[repr(C)]
#[derive(Debug)]
pub struct Ref {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Ref")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Ref => "System.Xml"."Ref"
);
#[cfg(feature = "System+Xml+Ref")]
impl std::ops::Deref for crate::System::Xml::Ref {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ref")]
impl std::ops::DerefMut for crate::System::Xml::Ref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ref")]
impl crate::System::Xml::Ref {}
#[cfg(feature = "System+Xml+Ref")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Ref {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
