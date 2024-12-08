#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Helper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Helper =>
    "System.Security.Cryptography.X509Certificates"."X509Helper"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl crate::System::Security::Cryptography::X509Certificates::X509Helper {}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Helper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
