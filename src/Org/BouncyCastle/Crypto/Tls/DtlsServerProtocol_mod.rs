#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsServerProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol,
    pub mVerifyRequests: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol => "Org.BouncyCastle.Crypto.Tls"
    ."DtlsServerProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState"
    )]
    pub type ServerHandshakeState = crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState;
    pub fn AbortServerHandshake(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        recordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AbortServerHandshake", (state, recordLayer, alertDescription))?;
        Ok(__cordl_ret)
    }
    pub fn Accept(
        &mut self,
        server: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport = __cordl_object
            .invoke("Accept", (server, transport))?;
        Ok(__cordl_ret)
    }
    pub fn ExpectCertificateVerifyMessage(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExpectCertificateVerifyMessage", (state))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCertificateRequest(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        certificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateCertificateRequest", (state, certificateRequest))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCertificateStatus(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        certificateStatus: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateCertificateStatus", (state, certificateStatus))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateNewSessionTicket(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        newSessionTicket: *mut crate::Org::BouncyCastle::Crypto::Tls::NewSessionTicket,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateNewSessionTicket", (state, newSessionTicket))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateServerHello(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateServerHello", (state))?;
        Ok(__cordl_ret)
    }
    pub fn InvalidateSession(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateSession", (state))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn NotifyClientCertificate(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        clientCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyClientCertificate", (state, clientCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCertificateVerify(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        prepareFinishHash: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCertificateVerify", (state, body, prepareFinishHash))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessClientCertificate(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientCertificate", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessClientHello(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientHello", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessClientKeyExchange(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientKeyExchange", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessClientSupplementalData(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessClientSupplementalData", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ServerHandshake(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        recordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport = __cordl_object
            .invoke("ServerHandshake", (state, recordLayer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
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
    pub fn get_VerifyRequests(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_VerifyRequests", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_VerifyRequests(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_VerifyRequests", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsServerProtocol_ServerHandshakeState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub server: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
    pub serverContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl,
    pub tlsSession: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    pub sessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    pub sessionParametersBuilder: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    pub offeredCipherSuites: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub offeredCompressionMethods: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub clientExtensions: *mut crate::System::Collections::IDictionary,
    pub serverExtensions: *mut crate::System::Collections::IDictionary,
    pub resumedSession: bool,
    pub secure_renegotiation: bool,
    pub allowCertificateStatus: bool,
    pub expectSessionTicket: bool,
    pub keyExchange: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    pub serverCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    pub certificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    pub clientCertificateType: i16,
    pub clientCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsServerProtocol/ServerHandshakeState"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
