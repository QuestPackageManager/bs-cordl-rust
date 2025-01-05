#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509Certificate2ImplMono {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplUnix,
    pub intermediateCerts: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509CertificateImplCollection,
    >,
    pub _cert: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono =>
    "System.Security.Cryptography.X509Certificates"."X509Certificate2ImplMono"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplUnix;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono {
    pub fn Clone(
        &mut self,
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
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDSAPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::DSA,
        > = __cordl_object.invoke("GetDSAPrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRSAPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = __cordl_object.invoke("GetRSAPrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawCertData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetRawCertData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportPkcs12_Il2CppString1(
        &mut self,
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Certificate,
        > = __cordl_object.invoke("ImportPkcs12", (rawData, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportPkcs12_SafePasswordHandle0(
        &mut self,
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Certificate,
        > = __cordl_object.invoke("ImportPkcs12", (rawData, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray_SafePasswordHandle_X509KeyStorageFlags2(
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rawData, password, keyStorageFlags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509Certificate0(
        cert: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cert))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509Certificate2ImplMono1(
        other: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object.into())
    }
    pub fn Verify(
        &mut self,
        thisCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (thisCertificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_SafePasswordHandle_X509KeyStorageFlags2(
        &mut self,
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafePasswordHandle,
        >,
        keyStorageFlags: crate::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rawData, password, keyStorageFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509Certificate0(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509Certificate2ImplMono1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Cert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Certificate,
        > = __cordl_object.invoke("get_Cert", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasPrivateKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasPrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IntermediateCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImplCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateImplCollection,
        > = __cordl_object.invoke("get_IntermediateCertificates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonoCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Certificate,
        > = __cordl_object.invoke("get_MonoCertificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        > = __cordl_object.invoke("get_PrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PrivateKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrivateKey", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509Certificate2ImplMono"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509Certificate2ImplMono {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
