#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsServerProtocol {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol,
    pub mVerifyRequests: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "DtlsServerProtocol";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::DtlsProtocol;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
                            >,
                            u8,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AbortServerHandshake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AbortServerHandshake", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (state, recordLayer, alertDescription))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Accept(
        &mut self,
        server: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
        >,
        transport: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
                        >,
                        2usize,
                    >("Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (server, transport))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExpectCertificateVerifyMessage(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                        >),
                        bool,
                        1usize,
                    >("ExpectCertificateVerifyMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpectCertificateVerifyMessage", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertificateRequest(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        certificateRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        2usize,
                    >("GenerateCertificateRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCertificateRequest", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, certificateRequest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCertificateStatus(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        certificateStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::CertificateStatus,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        2usize,
                    >("GenerateCertificateStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCertificateStatus", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, certificateStatus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateNewSessionTicket(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        newSessionTicket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::NewSessionTicket,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::NewSessionTicket,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        2usize,
                    >("GenerateNewSessionTicket")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateNewSessionTicket", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, newSessionTicket))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateServerHello(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        1usize,
                    >("GenerateServerHello")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateServerHello", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (state))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateSession(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InvalidateSession")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvalidateSession", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state))?
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
    pub fn NotifyClientCertificate(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        clientCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("NotifyClientCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyClientCertificate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, clientCertificate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertificateVerify(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        prepareFinishHash: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessCertificateVerify")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertificateVerify", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, body, prepareFinishHash))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessClientCertificate(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessClientCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessClientCertificate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, body))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessClientHello(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessClientHello")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessClientHello", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, body))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessClientKeyExchange(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessClientKeyExchange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessClientKeyExchange", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, body))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessClientSupplementalData(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        body: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessClientSupplementalData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessClientSupplementalData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, body))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ServerHandshake(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
        >,
        recordLayer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
                        >,
                        2usize,
                    >("ServerHandshake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ServerHandshake", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsTransport,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (state, recordLayer))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (secureRandom))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_VerifyRequests(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_VerifyRequests")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_VerifyRequests", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_VerifyRequests(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_VerifyRequests")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_VerifyRequests", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
    pub server: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsServer,
    >,
    pub serverContext: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsServerContextImpl,
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
    pub offeredCompressionMethods: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub clientExtensions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::IDictionary,
    >,
    pub serverExtensions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::IDictionary,
    >,
    pub resumedSession: bool,
    pub secure_renegotiation: bool,
    pub allowCertificateStatus: bool,
    pub expectSessionTicket: bool,
    pub keyExchange: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    >,
    pub serverCredentials: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    >,
    pub certificateRequest: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
    >,
    pub clientCertificateType: i16,
    pub clientCertificate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "DtlsServerProtocol/ServerHandshakeState";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsServerProtocol+ServerHandshakeState")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsServerProtocol_ServerHandshakeState {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
