#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSrpKeyExchange {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsKeyExchange,
    pub mTlsSigner: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
    >,
    pub mGroupVerifier: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
    >,
    pub mIdentity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mPassword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mServerPublicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
    pub mSrpGroup: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    >,
    pub mSrpClient: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Client,
    >,
    pub mSrpServer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Server,
    >,
    pub mSrpPeerCredentials: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub mSrpVerifier: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub mSrpSalt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mServerCredentials: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpKeyExchange")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpKeyExchange {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "TlsSrpKeyExchange";
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
    pub fn CreateSigner(
        keyExchange: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsSigner>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
                >,
                1usize,
            >("CreateSigner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateSigner", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
        > = unsafe { method.invoke_unchecked((), (keyExchange)) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateClientKeyExchange(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GenerateClientKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateClientKeyExchange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (output))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePremasterSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GeneratePremasterSecret")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GeneratePremasterSecret", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateServerKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GenerateServerKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateServerKeyExchange", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitVerifyer(
        &mut self,
        tlsSigner: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
        >,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
        securityParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
                3usize,
            >("InitVerifyer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitVerifyer", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISigner,
        > = unsafe {
            method.invoke_unchecked(self, (tlsSigner, algorithm, securityParameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, password),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_TlsSrpLoginParameters2(
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        loginParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyExchange, supportedSignatureAlgorithms, identity, loginParameters),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray1(
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        groupVerifier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn ProcessClientCredentials(
        &mut self,
        clientCredentials: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessClientCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessClientCredentials", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clientCredentials))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessClientKeyExchange(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessClientKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessClientKeyExchange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerCertificate(
        &mut self,
        serverCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::Certificate,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessServerCertificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerCertificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serverCertificate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerCredentials(
        &mut self,
        serverCredentials: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessServerCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerCredentials", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serverCredentials))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessServerKeyExchange(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessServerKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessServerKeyExchange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SkipServerCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("SkipServerCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SkipServerCredentials", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificateRequest(
        &mut self,
        certificateRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::CertificateRequest,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ValidateCertificateRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ValidateCertificateRequest", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (certificateRequest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (keyExchange, supportedSignatureAlgorithms, identity, password),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_TlsSrpLoginParameters2(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        loginParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        keyExchange,
                        supportedSignatureAlgorithms,
                        identity,
                        loginParameters,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TlsSrpGroupVerifier_Il2CppArray_Il2CppArray1(
        &mut self,
        keyExchange: i32,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        groupVerifier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
        >,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        keyExchange,
                        supportedSignatureAlgorithms,
                        groupVerifier,
                        identity,
                        password,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresServerKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_RequiresServerKeyExchange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_RequiresServerKeyExchange", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
