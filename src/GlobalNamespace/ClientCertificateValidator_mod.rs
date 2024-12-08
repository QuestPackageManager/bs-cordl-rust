#[cfg(feature = "ClientCertificateValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientCertificateValidator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ClientCertificateValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ClientCertificateValidator =>
    ""."ClientCertificateValidator"
);
#[cfg(feature = "ClientCertificateValidator")]
impl std::ops::Deref for crate::GlobalNamespace::ClientCertificateValidator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl std::ops::DerefMut for crate::GlobalNamespace::ClientCertificateValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl crate::GlobalNamespace::ClientCertificateValidator {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ValidateCertificateChain(
        &mut self,
        endPoint: *mut crate::GlobalNamespace::DnsEndPoint,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        certificateChain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateCertificateChain",
                (endPoint, certificate, certificateChain),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificateChainInternal(
        &mut self,
        endPoint: *mut crate::GlobalNamespace::DnsEndPoint,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        certificateChain: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateCertificateChainInternal",
                (endPoint, certificate, certificateChain),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ClientCertificateValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
