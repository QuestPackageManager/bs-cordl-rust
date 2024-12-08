#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::KeyHelper =>
    "System.Xml.Serialization"."KeyHelper"
);
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl std::ops::Deref for crate::System::Xml::Serialization::KeyHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::KeyHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl crate::System::Xml::Serialization::KeyHelper {}
#[cfg(feature = "System+Xml+Serialization+KeyHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::KeyHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
