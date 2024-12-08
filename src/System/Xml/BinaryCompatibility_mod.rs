#[cfg(feature = "System+Xml+BinaryCompatibility")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryCompatibility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+BinaryCompatibility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::BinaryCompatibility => "System.Xml"
    ."BinaryCompatibility"
);
#[cfg(feature = "System+Xml+BinaryCompatibility")]
impl std::ops::Deref for crate::System::Xml::BinaryCompatibility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinaryCompatibility")]
impl std::ops::DerefMut for crate::System::Xml::BinaryCompatibility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinaryCompatibility")]
impl crate::System::Xml::BinaryCompatibility {}
#[cfg(feature = "System+Xml+BinaryCompatibility")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::BinaryCompatibility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
