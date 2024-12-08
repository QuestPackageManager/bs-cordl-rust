#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeTranslator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::TypeTranslator =>
    "System.Xml.Serialization"."TypeTranslator"
);
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl std::ops::Deref for crate::System::Xml::Serialization::TypeTranslator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::TypeTranslator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl crate::System::Xml::Serialization::TypeTranslator {}
#[cfg(feature = "System+Xml+Serialization+TypeTranslator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::TypeTranslator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
