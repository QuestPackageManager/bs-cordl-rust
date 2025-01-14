#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpKeyRingGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    pub hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub certificationLevel: i32,
    pub rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub useSha1: bool,
    pub masterKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
    >,
    pub hashedPacketVector: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
    >,
    pub unhashedPacketVector: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
    >,
    pub _cordl_rand: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpKeyRingGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyRingGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyRingGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSubKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSubKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keyPair, hashAlgorithm))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSubKey_PgpKeyPair0(
        &mut self,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddSubKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSubKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keyPair))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSubKey_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector2(
        &mut self,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddSubKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSubKey", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keyPair, hashedPackets, unhashedPackets))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSubKey_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_HashAlgorithmTag3(
        &mut self,
        keyPair: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
        >,
        hashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        unhashedPackets: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
        >,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSignatureSubpacketVector,
                    >,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("AddSubKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSubKey", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (keyPair, hashedPackets, unhashedPackets, hashAlgorithm),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePublicKeyRing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyRing,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyRing,
                >,
                0usize,
            >("GeneratePublicKeyRing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GeneratePublicKeyRing", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyRing,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSecretKeyRing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKeyRing,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKeyRing,
                >,
                0usize,
            >("GenerateSecretKeyRing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateSecretKeyRing", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpSecretKeyRing,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn New_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn New_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom0(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
                    masterKey,
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
    pub fn New_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom1(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom4(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
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
                    self, ".ctor", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HashAlgorithmTag_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom6(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
                    self, ".ctor", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HashAlgorithmTag__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom5(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
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
                    self, ".ctor", 11usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom0(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
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
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom1(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
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
                    self, ".ctor", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom3(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
                    self, ".ctor", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppArray__cordl_bool_PgpSignatureSubpacketVector_PgpSignatureSubpacketVector_SecureRandom2(
        &mut self,
        certificationLevel: i32,
        masterKey: quest_hook::libil2cpp::Gc<
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
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
                    self, ".ctor", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
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
