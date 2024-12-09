#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Helper2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Helper2 =>
    "System.Security.Cryptography.X509Certificates"."X509Helper2"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl crate::System::Security::Cryptography::X509Certificates::X509Helper2 {}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509Helper2")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Helper2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
