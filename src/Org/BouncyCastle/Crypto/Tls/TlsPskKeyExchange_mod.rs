#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsPskKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange,
    pub mPskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    pub mPskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
    pub mDHVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
    pub mDHParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    pub mNamedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub mClientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPskIdentityHint: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPsk: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mDHAgreePrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
    pub mDHAgreePublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHPublicKeyParameters,
    pub mECAgreePrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mECAgreePublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    pub mServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub mRsaServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    pub mServerCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials,
    pub mPremasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsPskKeyExchange => "Org.BouncyCastle.Crypto.Tls"
    ."TlsPskKeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsPskKeyExchange {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsPskKeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsPskKeyExchange {
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
    pub fn GenerateOtherSecret(
        &mut self,
        pskLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateOtherSecret", (pskLength))?;
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
    pub fn New_DHParameters_Il2CppArray0(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        namedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        clientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    keyExchange,
                    supportedSignatureAlgorithms,
                    pskIdentity,
                    pskIdentityManager,
                    dhParameters,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TlsDHVerifier_DHParameters_Il2CppArray1(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
        namedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        clientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        serverECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    keyExchange,
                    supportedSignatureAlgorithms,
                    pskIdentity,
                    pskIdentityManager,
                    dhVerifier,
                    dhParameters,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
        Ok(__cordl_object)
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
    pub fn _ctor_DHParameters_Il2CppArray0(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
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
                    pskIdentity,
                    pskIdentityManager,
                    dhParameters,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsDHVerifier_DHParameters_Il2CppArray1(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
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
                    pskIdentity,
                    pskIdentityManager,
                    dhVerifier,
                    dhParameters,
                    namedCurves,
                    clientECPointFormats,
                    serverECPointFormats,
                ),
            )?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsPskKeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsPskKeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
