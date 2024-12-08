#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsRsaKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange,
    pub mServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub mRsaServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    pub mServerCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials,
    pub mPremasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsRsaKeyExchange => "Org.BouncyCastle.Crypto.Tls"
    ."TlsRsaKeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsRsaKeyExchange {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsRsaKeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsRsaKeyExchange {
    pub fn ProcessClientKeyExchange(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientKeyExchange", (input))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessClientCredentials(
        &mut self,
        clientCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientCredentials", (clientCredentials))?;
        Ok(__cordl_ret)
    }
    pub fn SkipServerCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipServerCredentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificateRequest(
        &mut self,
        certificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateCertificateRequest", (certificateRequest))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateClientKeyExchange(
        &mut self,
        output: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateClientKeyExchange", (output))?;
        Ok(__cordl_ret)
    }
    pub fn GeneratePremasterSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GeneratePremasterSecret", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateRsaPublicKey(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters = __cordl_object
            .invoke("ValidateRsaPublicKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerCredentials(
        &mut self,
        serverCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerCredentials", (serverCredentials))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerCertificate(
        &mut self,
        serverCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerCertificate", (serverCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (supportedSignatureAlgorithms))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (supportedSignatureAlgorithms))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsRsaKeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsRsaKeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
