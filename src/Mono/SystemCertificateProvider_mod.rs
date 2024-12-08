#[cfg(feature = "Mono+SystemCertificateProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemCertificateProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::SystemCertificateProvider => "Mono"
    ."SystemCertificateProvider"
);
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl std::ops::Deref for crate::Mono::SystemCertificateProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl std::ops::DerefMut for crate::Mono::SystemCertificateProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl crate::Mono::SystemCertificateProvider {
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
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl = __cordl_object
            .invoke("Import", (data, password, keyStorageFlags, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn Import_X509Certificate_CertificateImportFlags2(
        &mut self,
        cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl = __cordl_object
            .invoke("Import", (cert, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn Mono_ISystemCertificateProvider_Import_Il2CppArray_SafePasswordHandle_X509KeyStorageFlags_CertificateImportFlags0(
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
            .invoke(
                "Mono.ISystemCertificateProvider.Import",
                (data, password, keyStorageFlags, importFlags),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Mono_ISystemCertificateProvider_Import_X509Certificate_CertificateImportFlags1(
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
            .invoke("Mono.ISystemCertificateProvider.Import", (cert, importFlags))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_X509Pal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::X509PalImpl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::X509PalImpl = __cordl_object
            .invoke("get_X509Pal", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::SystemCertificateProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
