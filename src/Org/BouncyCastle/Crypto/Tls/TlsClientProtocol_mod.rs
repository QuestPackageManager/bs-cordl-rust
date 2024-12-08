#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsClientProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
    pub mTlsClient: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
    pub mTlsClientContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl,
    pub mSelectedSessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mKeyExchange: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    pub mAuthentication: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    pub mCertificateStatus: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
    pub mCertificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol => "Org.BouncyCastle.Crypto.Tls"
    ."TlsClientProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol {
    pub fn CleanupHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupHandshake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Connect(
        &mut self,
        tlsClient: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Connect", (tlsClient))?;
        Ok(__cordl_ret)
    }
    pub fn HandleHandshakeMessage(
        &mut self,
        _cordl_type: u8,
        buf: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHandshakeMessage", (_cordl_type, buf))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSupplementalData(
        &mut self,
        serverSupplementalData: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSupplementalData", (serverSupplementalData))?;
        Ok(__cordl_ret)
    }
    pub fn New_SecureRandom2(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_SecureRandom0(
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Stream_SecureRandom1(
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn ReceiveNewSessionTicketMessage(
        &mut self,
        buf: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveNewSessionTicketMessage", (buf))?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveServerHelloMessage(
        &mut self,
        buf: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveServerHelloMessage", (buf))?;
        Ok(__cordl_ret)
    }
    pub fn SendCertificateVerifyMessage(
        &mut self,
        certificateVerify: *mut crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendCertificateVerifyMessage", (certificateVerify))?;
        Ok(__cordl_ret)
    }
    pub fn SendClientHelloMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendClientHelloMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendClientKeyExchangeMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendClientKeyExchangeMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom2(
        &mut self,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_SecureRandom0(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Stream_SecureRandom1(
        &mut self,
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext = __cordl_object
            .invoke("get_Context", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContextAdmin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext = __cordl_object
            .invoke("get_ContextAdmin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Peer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer = __cordl_object
            .invoke("get_Peer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsClientProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsClientProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
