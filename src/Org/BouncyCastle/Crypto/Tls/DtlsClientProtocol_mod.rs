#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsClientProtocol_ClientHandshakeState {
    __cordl_parent: crate::System::Object,
    pub client: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
    pub clientContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl,
    pub tlsSession: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    pub sessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    pub sessionParametersBuilder: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    pub offeredCipherSuites: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub clientExtensions: *mut crate::System::Collections::IDictionary,
    pub serverExtensions: *mut crate::System::Collections::IDictionary,
    pub selectedSessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub resumedSession: bool,
    pub secure_renegotiation: bool,
    pub allowCertificateStatus: bool,
    pub expectSessionTicket: bool,
    pub keyExchange: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    pub authentication: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    pub certificateStatus: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
    pub certificateRequest: *mut crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    pub clientCredentials: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsClientProtocol/ClientHandshakeState"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsClientProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol => "Org.BouncyCastle.Crypto.Tls"
    ."DtlsClientProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol {
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState"
    )]
    pub type ClientHandshakeState = crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState;
    pub fn ProcessHelloVerifyRequest(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ProcessHelloVerifyRequest", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerKeyExchange(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerKeyExchange", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerSupplementalData(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerSupplementalData", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ReportServerVersion(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        server_version: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportServerVersion", (state, server_version))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateClientKeyExchange(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateClientKeyExchange", (state))?;
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
    pub fn InvalidateSession(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateSession", (state))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCertificateRequest(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCertificateRequest", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCertificateVerify(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        certificateVerify: *mut crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateCertificateVerify", (state, certificateVerify))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCertificateStatus(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCertificateStatus", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNewSessionTicket(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewSessionTicket", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerHello(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerHello", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateClientHello(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        client: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateClientHello", (state, client))?;
        Ok(__cordl_ret)
    }
    pub fn Connect(
        &mut self,
        client: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport = __cordl_object
            .invoke("Connect", (client, transport))?;
        Ok(__cordl_ret)
    }
    pub fn ClientHandshake(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        recordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport = __cordl_object
            .invoke("ClientHandshake", (state, recordLayer))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerCertificate(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate = __cordl_object
            .invoke("ProcessServerCertificate", (state, body))?;
        Ok(__cordl_ret)
    }
    pub fn AbortClientHandshake(
        &mut self,
        state: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        recordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AbortClientHandshake", (state, recordLayer, alertDescription))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
