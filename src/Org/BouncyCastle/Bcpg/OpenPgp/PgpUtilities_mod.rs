#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    pub const ReadAhead: i32 = 60i32;
    pub fn CreateWrapper(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IWrapper>,
                1usize,
            >("CreateWrapper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "CreateWrapper", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IWrapper,
        > = unsafe { method.invoke_unchecked((), (encAlgorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn DoMakeKeyFromPassPhrase(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                4usize,
            >("DoMakeKeyFromPassPhrase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "DoMakeKeyFromPassPhrase",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe {
            method
                .invoke_unchecked((), (algorithm, s2k, rawPassPhrase, clearPassPhrase))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DsaSigToMpi(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::MPInteger,
                        >,
                    >,
                >,
                1usize,
            >("DsaSigToMpi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "DsaSigToMpi", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        > = unsafe { method.invoke_unchecked((), (encoding))? };
        Ok(__cordl_ret.into())
    }
    pub fn EncodePassPhrase(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        utf8: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("EncodePassPhrase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "EncodePassPhrase", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (passPhrase, utf8))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateIV(
        length: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("GenerateIV")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateIV", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (length, random))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateS2k(
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        s2kCount: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                3usize,
            >("GenerateS2k")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateS2k", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::S2k,
        > = unsafe { method.invoke_unchecked((), (hashAlgorithm, s2kCount, random))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDecoderStream(
        inputStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                1usize,
            >("GetDecoderStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GetDecoderStream", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked((), (inputStream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDigestName(
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetDigestName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GetDigestName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (hashAlgorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag),
                i32,
                1usize,
            >("GetKeySize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GetKeySize", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (algorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSignatureName(
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetSignatureName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GetSignatureName", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (keyAlgorithm, hashAlgorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSymmetricCipherName(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetSymmetricCipherName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "GetSymmetricCipherName",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (algorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPossiblyBase64(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("IsPossiblyBase64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "IsPossiblyBase64", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeKey(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        keyBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                2usize,
            >("MakeKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "MakeKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked((), (algorithm, keyBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhrase(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                3usize,
            >("MakeKeyFromPassPhrase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "MakeKeyFromPassPhrase",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked((), (algorithm, s2k, passPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhraseRaw(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                3usize,
            >("MakeKeyFromPassPhraseRaw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "MakeKeyFromPassPhraseRaw",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked((), (algorithm, s2k, rawPassPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeKeyFromPassPhraseUtf8(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                3usize,
            >("MakeKeyFromPassPhraseUtf8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "MakeKeyFromPassPhraseUtf8",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked((), (algorithm, s2k, passPhrase))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeRandomKey(
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                2usize,
            >("MakeRandomKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "MakeRandomKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked((), (algorithm, random))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PipeFileContents(
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        pOut: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PipeFileContents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "PipeFileContents", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (file, pOut, bufSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RsaSigToMpi(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::MPInteger,
                        >,
                    >,
                >,
                1usize,
            >("RsaSigToMpi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "RsaSigToMpi", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::MPInteger>,
            >,
        > = unsafe { method.invoke_unchecked((), (encoding))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteFileToLiteralData_Il2CppArray1(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        fileType: char,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    char,
                    quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteFileToLiteralData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "WriteFileToLiteralData",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (output, fileType, file, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteFileToLiteralData_Stream__cordl_char_FileInfo0(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        fileType: char,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    char,
                    quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteFileToLiteralData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "WriteFileToLiteralData",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (output, fileType, file))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
