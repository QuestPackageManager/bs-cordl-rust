#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSrpKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange,
    pub mTlsSigner: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
    pub mGroupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
    pub mIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPassword: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub mSrpGroup: *mut crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    pub mSrpClient: *mut crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client,
    pub mSrpServer: *mut crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Server,
    pub mSrpPeerCredentials: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub mSrpVerifier: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub mSrpSalt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange => "Org.BouncyCastle.Crypto.Tls"
    ."TlsSrpKeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange {
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
    pub fn InitVerifyer(
        &mut self,
        tlsSigner: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        securityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("InitVerifyer", (tlsSigner, algorithm, securityParameters))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, password),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_TlsSrpLoginParameters2(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        loginParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, loginParameters),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray1(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        groupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    keyExchange,
                    supportedSignatureAlgorithms,
                    groupVerifier,
                    identity,
                    password,
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
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, password),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_TlsSrpLoginParameters2(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        loginParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, loginParameters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray1(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        groupVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        identity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
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
                    groupVerifier,
                    identity,
                    password,
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
