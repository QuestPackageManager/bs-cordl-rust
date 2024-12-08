#[cfg(feature = "Mono+ISystemCertificateProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ISystemCertificateProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+ISystemCertificateProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::ISystemCertificateProvider => "Mono"
    ."ISystemCertificateProvider"
);
#[cfg(feature = "Mono+ISystemCertificateProvider")]
impl std::ops::Deref for crate::Mono::ISystemCertificateProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+ISystemCertificateProvider")]
impl std::ops::DerefMut for crate::Mono::ISystemCertificateProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+ISystemCertificateProvider")]
impl crate::Mono::ISystemCertificateProvider {
    pub fn Import_Il2CppArray_CertificateImportFlags0(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl = __cordl_object
            .invoke("Import", (data, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn Import_Il2CppArray_SafePasswordHandle_X509KeyStorageFlags_CertificateImportFlags1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl = __cordl_object
            .invoke("Import", (data, password, keyStorageFlags, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn Import_X509Certificate_CertificateImportFlags2(
        &mut self,
        cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl = __cordl_object
            .invoke("Import", (cert, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Mono+ISystemCertificateProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::ISystemCertificateProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
