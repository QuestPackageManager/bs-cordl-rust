#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsDheKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsDHKeyExchange,
    pub mServerCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsDheKeyExchange => "Org.BouncyCastle.Crypto.Tls"
    ."TlsDheKeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsDheKeyExchange {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsDHKeyExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsDheKeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsDheKeyExchange {
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
    pub fn New_DHParameters0(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, dhParameters),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TlsDHVerifier_DHParameters1(
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, dhVerifier, dhParameters),
            )?;
        Ok(__cordl_object)
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
    pub fn _ctor_DHParameters0(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyExchange, supportedSignatureAlgorithms, dhParameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsDHVerifier_DHParameters1(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: *mut crate::System::Collections::IList,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        dhParameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, dhVerifier, dhParameters),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDheKeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsDheKeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
