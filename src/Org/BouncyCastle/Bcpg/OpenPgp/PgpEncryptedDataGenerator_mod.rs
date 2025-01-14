#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpEncryptedDataGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pOut: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    >,
    pub cOut: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::CipherStream,
    >,
    pub c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    pub withIntegrityPacket: bool,
    pub oldFormat: bool,
    pub digestOut: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IO::DigestStream,
    >,
    pub methods: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub defAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    pub _cordl_rand: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpEncryptedDataGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    #[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
    pub type EncMethod = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod;
    #[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
    pub type PbeMethod = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod;
    #[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
    pub type PubMethod = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod;
    pub fn AddCheckSum(
        &mut self,
        sessionInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddCheckSum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCheckSum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sessionInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethodRaw(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        s2kDigest: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddMethodRaw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethodRaw", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rawPassPhrase, s2kDigest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethodUtf8(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        s2kDigest: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddMethodUtf8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethodUtf8", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (passPhrase, s2kDigest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethod_Il2CppArray0(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (passPhrase))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethod_Il2CppArray_HashAlgorithmTag1(
        &mut self,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        s2kDigest: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (passPhrase, s2kDigest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethod_PgpPublicKey2(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMethod_PgpPublicKey__cordl_bool3(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        sessionKeyObfuscation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMethod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key, sessionKeyObfuscation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Close", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSessionInfo(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("CreateSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateSessionInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (algorithm, key)) };
        Ok(__cordl_ret.into())
    }
    pub fn DoAddMethod(
        &mut self,
        rawPassPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        clearPassPhrase: bool,
        s2kDigest: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    bool,
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DoAddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoAddMethod", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rawPassPhrase, clearPassPhrase, s2kDigest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_SecureRandom2(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm, _cordl_rand))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom__cordl_bool4(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm, _cordl_rand, oldFormat))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SymmetricKeyAlgorithmTag0(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        withIntegrityPacket: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm, withIntegrityPacket))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_SecureRandom3(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        withIntegrityPacket: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm, withIntegrityPacket, _cordl_rand))?;
        Ok(__cordl_object.into())
    }
    pub fn Open_Il2CppArray2(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                2usize,
            >("Open")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Open", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (outStr, buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_i64_1(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        length: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, i64),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                2usize,
            >("Open")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Open", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (outStr, length))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_i64_Il2CppArray0(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        length: i64,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                    i64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                3usize,
            >("Open")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Open", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (outStr, length, buffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom2(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encAlgorithm, _cordl_rand))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom__cordl_bool4(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encAlgorithm, _cordl_rand, oldFormat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SymmetricKeyAlgorithmTag0(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag),
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
            method.invoke_unchecked(self, (encAlgorithm))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        withIntegrityPacket: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encAlgorithm, withIntegrityPacket))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_SecureRandom3(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        withIntegrityPacket: bool,
        _cordl_rand: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (encAlgorithm, withIntegrityPacket, _cordl_rand))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl AsRef<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator")]
impl AsMut<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpEncryptedDataGenerator_EncMethod {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::ContainedPacket,
    pub sessionInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    pub key: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpEncryptedDataGenerator/EncMethod";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod {
    type Target = crate::Org::BouncyCastle::Bcpg::ContainedPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod {
    pub fn AddSessionInfo(
        &mut self,
        si: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSessionInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (si, random))
        };
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+EncMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpEncryptedDataGenerator_PbeMethod {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod,
    pub s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpEncryptedDataGenerator/PbeMethod";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod {
    pub fn AddSessionInfo(
        &mut self,
        si: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSessionInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (si, random))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        pOut: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::BcpgOutputStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Encode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Encode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pOut))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                >,
                0usize,
            >("GetKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetKey", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encAlgorithm, s2k, key))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encAlgorithm, s2k, key))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PbeMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PbeMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpEncryptedDataGenerator_PubMethod {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod,
    pub pubKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    >,
    pub sessionKeyObfuscation: bool,
    pub data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpEncryptedDataGenerator/PubMethod";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_EncMethod;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod {
    pub fn AddSessionInfo(
        &mut self,
        sessionInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSessionInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sessionInfo, random))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToEncodedMpi(
        &mut self,
        encryptedSessionInfo: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("ConvertToEncodedMpi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertToEncodedMpi", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (encryptedSessionInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        pOut: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::BcpgOutputStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Encode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Encode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pOut))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptSessionInfo(
        &mut self,
        sessionInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("EncryptSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EncryptSessionInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (sessionInfo, random)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        sessionKeyObfuscation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKey, sessionKeyObfuscation))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessSessionInfo(
        &mut self,
        encryptedSessionInfo: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    >,
                >,
                1usize,
            >("ProcessSessionInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessSessionInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        > = unsafe { method.invoke_unchecked(self, (encryptedSessionInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        >,
        sessionKeyObfuscation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pubKey, sessionKeyObfuscation))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpEncryptedDataGenerator+PubMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedDataGenerator_PubMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
