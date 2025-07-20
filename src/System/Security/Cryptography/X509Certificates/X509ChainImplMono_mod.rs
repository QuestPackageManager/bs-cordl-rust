#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainImplMono {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
    pub location: crate::System::Security::Cryptography::X509Certificates::StoreLocation,
    pub elements: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
    >,
    pub policy: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
    >,
    pub status: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
        >,
    >,
    pub max_path_length: i32,
    pub working_issuer_name: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    >,
    pub working_public_key: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::AsymmetricAlgorithm,
    >,
    pub bce_restriction: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
    >,
    pub roots: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    >,
    pub cas: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    >,
    pub root_store: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Store,
    >,
    pub ca_store: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Store,
    >,
    pub user_root_store: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Store,
    >,
    pub user_ca_store: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Store,
    >,
    pub collection: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    >,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography.X509Certificates";
    const CLASS_NAME: &'static str = "X509ChainImplMono";
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
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509ChainImpl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    pub fn AddStatus(
        &mut self,
        error: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "AddStatus", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                bool,
                1usize,
            >("Build")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "Build", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildChainFrom(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
                1usize,
            >("BuildChainFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "BuildChainFrom", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = unsafe {
            method.invoke_unchecked(self, (certificate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrls(
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ski: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        store: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
                3usize,
            >("CheckCrls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "CheckCrls", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl,
        > = unsafe { method.invoke_unchecked((), (subject, ski, store))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRevocationOnChain(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckRevocationOnChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "CheckRevocationOnChain",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRevocation_X509Certificate2_1(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        ca_cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        online: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                    >,
                    bool,
                ),
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
                3usize,
            >("CheckRevocation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "CheckRevocation", 3usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = unsafe {
            method.invoke_unchecked(self, (certificate, ca_cert, online))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRevocation_i32_0(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        ca: i32,
        online: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                    >,
                    i32,
                    bool,
                ),
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
                3usize,
            >("CheckRevocation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "CheckRevocation", 3usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = unsafe {
            method.invoke_unchecked(self, (certificate, ca, online))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindCrl(
        &mut self,
        caCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
                1usize,
            >("FindCrl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "FindCrl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl,
        > = unsafe { method.invoke_unchecked(self, (caCertificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindParent(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >,
                1usize,
            >("FindParent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "FindParent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = unsafe { method.invoke_unchecked(self, (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Certificate2_0(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetAuthorityKeyIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetAuthorityKeyIdentifier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Crl1(
        crl: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetAuthorityKeyIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetAuthorityKeyIdentifier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (crl))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Extension2(
        ext: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Extension>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Extension>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetAuthorityKeyIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetAuthorityKeyIdentifier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (ext))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSubjectKeyIdentifier(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetSubjectKeyIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GetSubjectKeyIdentifier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsChainComplete(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                bool,
                1usize,
            >("IsChainComplete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "IsChainComplete", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSelfIssued(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >),
                bool,
                1usize,
            >("IsSelfIssued")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "IsSelfIssued", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSignedWith(
        &mut self,
        _cordl_signed: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        pubkey: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::AsymmetricAlgorithm,
                    >,
                ),
                bool,
                2usize,
            >("IsSignedWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "IsSignedWith", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (_cordl_signed, pubkey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        useMachineContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useMachineContext))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareForNextCertificate(
        &mut self,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PrepareForNextCertificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "PrepareForNextCertificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Process(
        &mut self,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Process")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "Process", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertificateExtensions(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ProcessCertificateExtensions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "ProcessCertificateExtensions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlEntryExtensions(
        &mut self,
        entry: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl_X509CrlEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Mono::Security::X509::X509Crl_X509CrlEntry,
                >),
                bool,
                1usize,
            >("ProcessCrlEntryExtensions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "ProcessCrlEntryExtensions", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (entry))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlExtensions(
        &mut self,
        crl: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>),
                bool,
                1usize,
            >("ProcessCrlExtensions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "ProcessCrlExtensions",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (crl))? };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectBestFromCollection(
        &mut self,
        child: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        c: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
                >,
                2usize,
            >("SelectBestFromCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "SelectBestFromCollection", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = unsafe { method.invoke_unchecked(self, (child, c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChain(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ValidateChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "ValidateChain", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flag))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WrapUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("WrapUp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "WrapUp", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        useMachineContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (useMachineContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateAuthorities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
                >,
                0usize,
            >("get_CertificateAuthorities")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_CertificateAuthorities", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
                >,
                0usize,
            >("get_CertificateCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_CertificateCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
                >,
                0usize,
            >("get_ChainElements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_ChainElements",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
                >,
                0usize,
            >("get_ChainPolicy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_ChainPolicy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
                    >,
                >,
                0usize,
            >("get_ChainStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_ChainStatus", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_IsValid", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_LMCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Store,
                >,
                0usize,
            >("get_LMCAStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_LMCAStore", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_LMRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Store,
                >,
                0usize,
            >("get_LMRootStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_LMRootStore", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Roots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
                >,
                0usize,
            >("get_Roots")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_Roots", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UserCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Store,
                >,
                0usize,
            >("get_UserCAStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_UserCAStore", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UserRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Store,
                >,
                0usize,
            >("get_UserRootStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono
                    as quest_hook::libil2cpp::Type > ::class(), "get_UserRootStore",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
