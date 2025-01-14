#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsClientProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "DtlsClientProtocol";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn AbortClientHandshake(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
                    >,
                    u8,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AbortClientHandshake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AbortClientHandshake", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, recordLayer, alertDescription))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClientHandshake(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
                >,
                2usize,
            >("ClientHandshake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClientHandshake", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
        > = unsafe { method.invoke_unchecked(self, (state, recordLayer)) };
        Ok(__cordl_ret.into())
    }
    pub fn Connect(
        &mut self,
        client: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
        >,
        transport: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
                >,
                2usize,
            >("Connect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Connect", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
        > = unsafe { method.invoke_unchecked(self, (client, transport)) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertificateVerify(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        certificateVerify: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DigitallySigned,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("GenerateCertificateVerify")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateCertificateVerify", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (state, certificateVerify)) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateClientHello(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        client: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("GenerateClientHello")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateClientHello", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (state, client)) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateClientKeyExchange(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("GenerateClientKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateClientKeyExchange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (state)) };
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateSession(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InvalidateSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InvalidateSession", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
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
    pub fn PatchClientHelloWithCookie(
        clientHelloBody: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        cookie: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("PatchClientHelloWithCookie")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PatchClientHelloWithCookie", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (clientHelloBody, cookie)) };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertificateRequest(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessCertificateRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessCertificateRequest", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertificateStatus(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessCertificateStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessCertificateStatus", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessHelloVerifyRequest(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("ProcessHelloVerifyRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessHelloVerifyRequest", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (state, body)) };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNewSessionTicket(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessNewSessionTicket")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessNewSessionTicket", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerCertificate(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Certificate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                >,
                2usize,
            >("ProcessServerCertificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerCertificate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        > = unsafe { method.invoke_unchecked(self, (state, body)) };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerHello(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessServerHello")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerHello", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerKeyExchange(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessServerKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerKeyExchange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerSupplementalData(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessServerSupplementalData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerSupplementalData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, body))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReportServerVersion(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
        >,
        server_version: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReportServerVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReportServerVersion", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, server_version))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Security::SecureRandom,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (secureRandom))
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsClientProtocol_ClientHandshakeState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub client: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsClient,
    >,
    pub clientContext: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsClientContextImpl,
    >,
    pub tlsSession: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    >,
    pub sessionParameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    >,
    pub sessionParametersBuilder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    >,
    pub offeredCipherSuites: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub clientExtensions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::IDictionary,
    >,
    pub serverExtensions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::IDictionary,
    >,
    pub selectedSessionID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub resumedSession: bool,
    pub secure_renegotiation: bool,
    pub allowCertificateStatus: bool,
    pub expectSessionTicket: bool,
    pub keyExchange: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    >,
    pub authentication: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    >,
    pub certificateStatus: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
    >,
    pub certificateRequest: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    >,
    pub clientCredentials: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "DtlsClientProtocol/ClientHandshakeState";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsClientProtocol+ClientHandshakeState")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsClientProtocol_ClientHandshakeState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
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
