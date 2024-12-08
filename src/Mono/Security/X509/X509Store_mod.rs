#[cfg(feature = "Mono+Security+X509+X509Store")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Store {
    __cordl_parent: crate::System::Object,
    pub _storePath: *mut crate::System::String,
    pub _certificates: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub _crls: *mut crate::System::Collections::ArrayList,
    pub _crl: bool,
    pub _newFormat: bool,
}
#[cfg(feature = "Mono+Security+X509+X509Store")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509Store =>
    "Mono.Security.X509"."X509Store"
);
#[cfg(feature = "Mono+Security+X509+X509Store")]
impl std::ops::Deref for crate::Mono::Security::X509::X509Store {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Store")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Store")]
impl crate::Mono::Security::X509::X509Store {
    pub fn BuildCertificatesCollection(
        &mut self,
        storeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509CertificateCollection = __cordl_object
            .invoke("BuildCertificatesCollection", (storeName))?;
        Ok(__cordl_ret)
    }
    pub fn BuildCrlsCollection(
        &mut self,
        storeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("BuildCrlsCollection", (storeName))?;
        Ok(__cordl_ret)
    }
    pub fn CheckStore(
        &mut self,
        path: *mut crate::System::String,
        throwException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckStore", (path, throwException))?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        filename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Load", (filename))?;
        Ok(__cordl_ret)
    }
    pub fn LoadCertificate(
        &mut self,
        filename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Certificate = __cordl_object
            .invoke("LoadCertificate", (filename))?;
        Ok(__cordl_ret)
    }
    pub fn LoadCrl(
        &mut self,
        filename: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Crl = __cordl_object
            .invoke("LoadCrl", (filename))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        path: *mut crate::System::String,
        crl: bool,
        newFormat: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, crl, newFormat))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        path: *mut crate::System::String,
        crl: bool,
        newFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, crl, newFormat))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509CertificateCollection = __cordl_object
            .invoke("get_Certificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Crls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_Crls", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+X509+X509Store")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X509Store {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
