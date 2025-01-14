#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpPublicKeyEncryptedData {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedData,
    pub keyData: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpPublicKeyEncryptedData";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData {
    pub fn ConfirmCheckSum(
        &mut self,
        sessionInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                bool,
                1usize,
            >("ConfirmCheckSum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConfirmCheckSum", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (sessionInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataStream(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                >),
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                1usize,
            >("GetDataStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDataStream", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (privKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyCipher(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                >,
                1usize,
            >("GetKeyCipher")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetKeyCipher", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = unsafe { method.invoke_unchecked((), (algorithm)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSymmetricAlgorithm(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                >),
                crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                1usize,
            >("GetSymmetricAlgorithm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSymmetricAlgorithm", 1usize
                )
            });
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = unsafe {
            method.invoke_unchecked(self, (privKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        keyData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
        >,
        encData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyData, encData))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessEncodedMpi(
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        >,
        _cordl_size: i32,
        mpiEnc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessEncodedMpi")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessEncodedMpi", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cipher, _cordl_size, mpiEnc))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecoverSessionData(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("RecoverSessionData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RecoverSessionData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (privKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
        >,
        encData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
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
            method.invoke_unchecked(self, (keyData, encData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_KeyId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_KeyId", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
