#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpPbeEncryptedData {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedData,
    pub keyData: *mut crate::Org::BouncyCastle::Bcpg::SymmetricKeyEncSessionPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpPbeEncryptedData =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpPbeEncryptedData"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPbeEncryptedData {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpEncryptedData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPbeEncryptedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPbeEncryptedData {
    pub fn CreateStreamCipher(
        &mut self,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBufferedCipher = __cordl_object
            .invoke("CreateStreamCipher", (keyAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn DoGetDataStream(
        &mut self,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        clearPassPhrase: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("DoGetDataStream", (rawPassPhrase, clearPassPhrase))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStream(
        &mut self,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetDataStream", (passPhrase))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStreamRaw(
        &mut self,
        rawPassPhrase: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetDataStreamRaw", (rawPassPhrase))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStreamUtf8(
        &mut self,
        passPhrase: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetDataStreamUtf8", (passPhrase))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetInputStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyData: *mut crate::Org::BouncyCastle::Bcpg::SymmetricKeyEncSessionPacket,
        encData: *mut crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyData, encData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        keyData: *mut crate::Org::BouncyCastle::Bcpg::SymmetricKeyEncSessionPacket,
        encData: *mut crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyData, encData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpPbeEncryptedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPbeEncryptedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
