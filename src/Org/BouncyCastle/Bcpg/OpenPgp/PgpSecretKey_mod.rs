#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpSecretKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub secret: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
    >,
    pub _cordl_pub: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpSecretKey";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    pub fn CertifiedPublicKey_HashAlgorithmTag1(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                        >,
                        6usize,
                    >("CertifiedPublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CertifiedPublicKey", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        hashedPackets,
                        unhashedPackets,
                        hashAlgorithm,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CertifiedPublicKey_i32_PgpKeyPair_Il2CppString_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector0(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                        >,
                        5usize,
                    >("CertifiedPublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CertifiedPublicKey", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (certificationLevel, keyPair, id, hashedPackets, unhashedPackets),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Checksum(
        useSha1: bool,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        3usize,
                    >("Checksum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Checksum", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (useSha1, bytes, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPassword(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        oldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        5usize,
                    >("CopyWithNewPassword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyWithNewPassword", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (key, oldPassPhrase, newPassPhrase, newEncAlgorithm, _cordl_rand),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPasswordRaw(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        rawOldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        rawNewPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        5usize,
                    >("CopyWithNewPasswordRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyWithNewPasswordRaw", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        key,
                        rawOldPassPhrase,
                        rawNewPassPhrase,
                        newEncAlgorithm,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyWithNewPasswordUtf8(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        oldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        5usize,
                    >("CopyWithNewPasswordUtf8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyWithNewPasswordUtf8", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (key, oldPassPhrase, newPassPhrase, newEncAlgorithm, _cordl_rand),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoCopyWithNewPassword(
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        rawOldPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        rawNewPassPhrase: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        clearPassPhrase: bool,
        newEncAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        6usize,
                    >("DoCopyWithNewPassword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoCopyWithNewPassword", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        key,
                        rawOldPassPhrase,
                        rawNewPassPhrase,
                        clearPassPhrase,
                        newEncAlgorithm,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoExtractPrivateKey(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                        >,
                        2usize,
                    >("DoExtractPrivateKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoExtractPrivateKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (rawPassPhrase, clearPassPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoParseSecretKeyFromSExpr_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        4usize,
                    >("DoParseSecretKeyFromSExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoParseSecretKeyFromSExpr", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (inputStream, rawPassPhrase, clearPassPhrase, pubKey),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoParseSecretKeyFromSExpr_Stream_Il2CppArray__cordl_bool1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        3usize,
                    >("DoParseSecretKeyFromSExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoParseSecretKeyFromSExpr", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (inputStream, rawPassPhrase, clearPassPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Encode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Encode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (outStr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptData(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataOff: i32,
        dataLen: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u8>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        7usize,
                    >("EncryptData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncryptData", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (encAlgorithm, key, data, dataOff, dataLen, random, iv),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKeyDataV3(
        rawKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        s2k: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Bcpg::S2k,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u8>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        7usize,
                    >("EncryptKeyDataV3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncryptKeyDataV3", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        rawKeyData,
                        encAlgorithm,
                        rawPassPhrase,
                        clearPassPhrase,
                        random,
                        s2k,
                        iv,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKeyDataV4(
        rawKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        s2k: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        >,
        iv: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Bcpg::S2k,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<u8>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        8usize,
                    >("EncryptKeyDataV4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncryptKeyDataV4", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        rawKeyData,
                        encAlgorithm,
                        hashAlgorithm,
                        rawPassPhrase,
                        clearPassPhrase,
                        random,
                        s2k,
                        iv,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractKeyData(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
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
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        2usize,
                    >("ExtractKeyData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractKeyData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (rawPassPhrase, clearPassPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKey(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<char>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                        >,
                        1usize,
                    >("ExtractPrivateKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractPrivateKey", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (passPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKeyRaw(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                        >,
                        1usize,
                    >("ExtractPrivateKeyRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractPrivateKeyRaw", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (rawPassPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractPrivateKeyUtf8(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<char>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                        >,
                        1usize,
                    >("ExtractPrivateKeyUtf8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractPrivateKeyUtf8", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (passPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDValue(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        curveName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        4usize,
                    >("GetDValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDValue", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (inputStream, rawPassPhrase, clearPassPhrase, curveName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetECKey(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
                        >,
                        2usize,
                    >("GetECKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetECKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (algorithm, bcpgIn))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        0usize,
                    >("GetEncoded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEncoded", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_PgpPrivateKey_PgpPublicKey_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_SecureRandom__cordl_bool1(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isMasterKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    privKey,
                    pubKey,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    _cordl_rand,
                    isMasterKey,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecretKeyPacket_PgpPublicKey0(
        secret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
        >,
        _cordl_pub: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secret, _cordl_pub))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom7(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom9(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom10(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom8(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    hashAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    clearPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    keyPair,
                    id,
                    encAlgorithm,
                    utf8PassPhrase,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom11(
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom12(
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    algorithm,
                    pubKey,
                    privKey,
                    _cordl_time,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseSecretKeyFromSExprRaw_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        3usize,
                    >("ParseSecretKeyFromSExprRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExprRaw", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, rawPassPhrase, pubKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprRaw_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        2usize,
                    >("ParseSecretKeyFromSExprRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExprRaw", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, rawPassPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprUtf8_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        3usize,
                    >("ParseSecretKeyFromSExprUtf8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExprUtf8", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, passPhrase, pubKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExprUtf8_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        2usize,
                    >("ParseSecretKeyFromSExprUtf8")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExprUtf8", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, passPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExpr_PgpPublicKey0(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        3usize,
                    >("ParseSecretKeyFromSExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExpr", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, passPhrase, pubKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSecretKeyFromSExpr_Stream_Il2CppArray1(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        2usize,
                    >("ParseSecretKeyFromSExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseSecretKeyFromSExpr", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (inputStream, passPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecoverKeyData(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        modeAndPadding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        7usize,
                    >("RecoverKeyData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RecoverKeyData", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (encAlgorithm, modeAndPadding, key, iv, keyData, keyOff, keyLen),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplacePublicKey(
        secretKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        >,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
                        >,
                        2usize,
                    >("ReplacePublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReplacePublicKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey,
        > = unsafe { cordl_method_info.invoke_unchecked((), (secretKey, publicKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PgpPrivateKey_PgpPublicKey_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_SecureRandom__cordl_bool1(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isMasterKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        privKey,
                        pubKey,
                        encAlgorithm,
                        rawPassPhrase,
                        clearPassPhrase,
                        useSha1,
                        _cordl_rand,
                        isMasterKey,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecretKeyPacket_PgpPublicKey0(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
        >,
        _cordl_pub: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (secret, _cordl_pub))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom7(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        hashAlgorithm,
                        passPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom9(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        hashAlgorithm,
                        rawPassPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom10(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        hashAlgorithm,
                        rawPassPhrase,
                        clearPassPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom8(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        hashAlgorithm,
                        utf8PassPhrase,
                        passPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        passPhrase,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        passPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        rawPassPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        rawPassPhrase,
                        clearPassPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PgpKeyPair_Il2CppString_SymmetricKeyAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        &mut self,
        certificationLevel: i32,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        keyPair,
                        id,
                        encAlgorithm,
                        utf8PassPhrase,
                        passPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom11(
        &mut self,
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        algorithm,
                        pubKey,
                        privKey,
                        _cordl_time,
                        id,
                        encAlgorithm,
                        passPhrase,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime_Il2CppString_SymmetricKeyAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom12(
        &mut self,
        certificationLevel: i32,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        _cordl_time: crate::System::DateTime,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        useSha1: bool,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        certificationLevel,
                        algorithm,
                        pubKey,
                        privKey,
                        _cordl_time,
                        id,
                        encAlgorithm,
                        passPhrase,
                        useSha1,
                        hashedPackets,
                        unhashedPackets,
                        _cordl_rand,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMasterKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsMasterKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsMasterKey", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPrivateKeyEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsPrivateKeyEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsPrivateKeyEmpty", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSigningKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsSigningKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsSigningKey", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyEncryptionAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                        0usize,
                    >("get_KeyEncryptionAlgorithm")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_KeyEncryptionAlgorithm", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("get_KeyId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_KeyId", 0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                        >,
                        0usize,
                    >("get_PublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PublicKey", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_S2k(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                        0usize,
                    >("get_S2k")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_S2k",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::S2k,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_S2kUsage(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_S2kUsage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_S2kUsage", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UserAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >,
                        0usize,
                    >("get_UserAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_UserAttributes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UserIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >,
                        0usize,
                    >("get_UserIds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_UserIds", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpSecretKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
