#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RSACertificateExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::RSACertificateExtensions =>
    "System.Security.Cryptography.X509Certificates"."RSACertificateExtensions"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::RSACertificateExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::RSACertificateExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
impl crate::System::Security::Cryptography::X509Certificates::RSACertificateExtensions {
    pub fn GetRSAPublicKey(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRSAPublicKey", (certificate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+RSACertificateExtensions"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::RSACertificateExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
