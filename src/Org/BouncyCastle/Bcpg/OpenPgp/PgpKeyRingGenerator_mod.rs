#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpKeyRingGenerator {
    __cordl_parent: crate::System::Object,
    pub keys: *mut crate::System::Collections::IList,
    pub id: *mut crate::System::String,
    pub encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    pub hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub certificationLevel: i32,
    pub rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub useSha1: bool,
    pub masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
    pub hashedPacketVector: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
    pub unhashedPacketVector: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
    pub _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpKeyRingGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    pub fn AddSubKey_HashAlgorithmTag1(
        &mut self,
        keyPair: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSubKey", (keyPair, hashAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn AddSubKey_PgpKeyPair0(
        &mut self,
        keyPair: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSubKey", (keyPair))?;
        Ok(__cordl_ret)
    }
    pub fn AddSubKey_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector2(
        &mut self,
        keyPair: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSubKey", (keyPair, hashedPackets, unhashedPackets))?;
        Ok(__cordl_ret)
    }
    pub fn AddSubKey_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_HashAlgorithmTag3(
        &mut self,
        keyPair: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSubKey",
                (keyPair, hashedPackets, unhashedPackets, hashAlgorithm),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GeneratePublicKeyRing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyRing,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyRing = __cordl_object
            .invoke("GeneratePublicKeyRing", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSecretKeyRing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKeyRing,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKeyRing = __cordl_object
            .invoke("GenerateSecretKeyRing", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_object)
    }
    pub fn New_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_object)
    }
    pub fn New_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom0(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom1(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_object)
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom0(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    passPhrase,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom1(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    passPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
                    id,
                    encAlgorithm,
                    rawPassPhrase,
                    useSha1,
                    hashedPackets,
                    unhashedPackets,
                    _cordl_rand,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        &mut self,
        certificationLevel: i32,
        masterKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        id: *mut crate::System::String,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        utf8PassPhrase: bool,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        useSha1: bool,
        hashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        unhashedPackets: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    certificationLevel,
                    masterKey,
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
