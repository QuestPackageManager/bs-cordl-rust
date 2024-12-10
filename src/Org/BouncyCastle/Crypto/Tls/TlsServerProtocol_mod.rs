#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsServerProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
    pub mTlsServer: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
    pub mTlsServerContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl,
    pub mKeyExchange: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    pub mServerCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    pub mCertificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    pub mClientCertificateType: i16,
    pub mPrepareFinishHash: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsServerProtocol => "Org.BouncyCastle.Crypto.Tls"
    ."TlsServerProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsServerProtocol {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsServerProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsServerProtocol {
    pub fn Accept(
        &mut self,
        tlsServer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Accept", (tlsServer))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupHandshake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectCertificateVerifyMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExpectCertificateVerifyMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAlertWarningMessage(
        &mut self,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAlertWarningMessage", (alertDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHandshakeMessage(
        &mut self,
        _cordl_type: u8,
        buf: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHandshakeMessage", (_cordl_type, buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_SecureRandom2(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_SecureRandom0(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, secureRandom))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_Stream_SecureRandom1(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyClientCertificate(
        &mut self,
        clientCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyClientCertificate", (clientCertificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveCertificateMessage(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveCertificateMessage", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveCertificateVerifyMessage(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveCertificateVerifyMessage", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveClientHelloMessage(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveClientHelloMessage", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveClientKeyExchangeMessage(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveClientKeyExchangeMessage", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendCertificateRequestMessage(
        &mut self,
        certificateRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendCertificateRequestMessage", (certificateRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendCertificateStatusMessage(
        &mut self,
        certificateStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendCertificateStatusMessage", (certificateStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendNewSessionTicketMessage(
        &mut self,
        newSessionTicket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::NewSessionTicket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNewSessionTicketMessage", (newSessionTicket))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendServerHelloDoneMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendServerHelloDoneMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendServerHelloMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendServerHelloMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendServerKeyExchangeMessage(
        &mut self,
        serverKeyExchange: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendServerKeyExchangeMessage", (serverKeyExchange))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom2(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_SecureRandom0(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, secureRandom))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_Stream_SecureRandom1(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        > = __cordl_object.invoke("get_Context", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContextAdmin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
        > = __cordl_object.invoke("get_ContextAdmin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Peer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsPeer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
        > = __cordl_object.invoke("get_Peer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsServerProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsServerProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
