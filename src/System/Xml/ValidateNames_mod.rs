#[cfg(feature = "System+Xml+ValidateNames")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidateNames {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+ValidateNames")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ValidateNames => "System.Xml"
    ."ValidateNames"
);
#[cfg(feature = "System+Xml+ValidateNames")]
impl std::ops::Deref for crate::System::Xml::ValidateNames {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ValidateNames")]
impl std::ops::DerefMut for crate::System::Xml::ValidateNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ValidateNames")]
impl crate::System::Xml::ValidateNames {}
#[cfg(feature = "System+Xml+ValidateNames")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::ValidateNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
