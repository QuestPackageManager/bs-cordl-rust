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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ConfirmCheckSum", (sessionInfo))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetDataStream", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyCipher(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyCipher", (algorithm))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = __cordl_object
            .invoke("GetSymmetricAlgorithm", (privKey))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessEncodedMpi", (cipher, _cordl_size, mpiEnc))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("RecoverSessionData", (privKey))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyData, encData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
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
