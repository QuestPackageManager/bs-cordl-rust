#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpCompressedData {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject,
    pub data: *mut crate::Org::BouncyCastle::Bcpg::CompressedDataPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedData =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpCompressedData"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedData {
    type Target = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedData {
    pub fn _ctor(
        &mut self,
        bcpgInput: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgInput))?;
        Ok(__cordl_ret)
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetDataStream", ())?;
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
        bcpgInput: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgInput))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
