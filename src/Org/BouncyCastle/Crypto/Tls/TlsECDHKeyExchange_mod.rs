#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsECDHKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange,
    pub mTlsSigner: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
    pub mNamedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub mClientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub mAgreementCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAgreementCredentials,
    pub mECAgreePrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mECAgreePublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsECDHKeyExchange => "Org.BouncyCastle.Crypto.Tls"
    ."TlsECDHKeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsECDHKeyExchange {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsECDHKeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsECDHKeyExchange {
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
    pub fn _ctor(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        namedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        clientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    keyExchange,
                    supportedSignatureAlgorithms,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
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
    pub fn get_RequiresServerKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequiresServerKeyExchange", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerKeyExchange(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerKeyExchange", (input))?;
        Ok(__cordl_ret)
    }
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
    pub fn GenerateServerKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateServerKeyExchange", ())?;
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
    pub fn ProcessClientCertificate(
        &mut self,
        clientCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientCertificate", (clientCertificate))?;
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
    pub fn Init(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (context))?;
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
    pub fn New(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        namedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        clientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    keyExchange,
                    supportedSignatureAlgorithms,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsECDHKeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsECDHKeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
