#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpPublicKeyEncryptedData {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedData,
    pub keyData: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPublicKeyEncryptedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKeyEncryptedData =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpPublicKeyEncryptedData"
);
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
        sessionInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ConfirmCheckSum", (sessionInfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStream(
        &mut self,
        privKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetDataStream", (privKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetSymmetricAlgorithm(
        &mut self,
        privKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = __cordl_object
            .invoke("GetSymmetricAlgorithm", (privKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyData: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
        encData: *mut crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyData, encData))?;
        Ok(__cordl_object)
    }
    pub fn RecoverSessionData(
        &mut self,
        privKey: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("RecoverSessionData", (privKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        keyData: *mut crate::Org::BouncyCastle::Bcpg::PublicKeyEncSessionPacket,
        encData: *mut crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyData, encData))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
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
