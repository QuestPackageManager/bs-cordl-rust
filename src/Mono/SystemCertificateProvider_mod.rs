#[cfg(feature = "Mono+SystemCertificateProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemCertificateProvider {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::SystemCertificateProvider => "Mono"
    ."SystemCertificateProvider"
);
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl std::ops::Deref for crate::Mono::SystemCertificateProvider {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn EnsureInitialized() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetX509Pal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetX509Pal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_CertificateImportFlags0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = __cordl_object.invoke("Import", (data, importFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_CertificateImportFlags2(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
        > = __cordl_object.invoke("Import", (cert, importFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Import_Gc_X509KeyStorageFlags_CertificateImportFlags1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Impl,
        > = __cordl_object
            .invoke("Import", (data, password, keyStorageFlags, importFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mono_ISystemCertificateProvider_Import_CertificateImportFlags1(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = __cordl_object
            .invoke("Mono.ISystemCertificateProvider.Import", (cert, importFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mono_ISystemCertificateProvider_Import_Gc_X509KeyStorageFlags_CertificateImportFlags0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
        importFlags: crate::Mono::CertificateImportFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImpl,
        > = __cordl_object
            .invoke(
                "Mono.ISystemCertificateProvider.Import",
                (data, password, keyStorageFlags, importFlags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_X509Pal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl> = __cordl_object
            .invoke("get_X509Pal", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider>>
for crate::Mono::SystemCertificateProvider {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+SystemCertificateProvider")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider>>
for crate::Mono::SystemCertificateProvider {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider> {
        unsafe { std::mem::transmute(self) }
    }
}
