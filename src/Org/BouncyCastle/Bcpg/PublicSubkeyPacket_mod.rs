#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct PublicSubkeyPacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::PublicSubkeyPacket =>
    "Org.BouncyCastle.Bcpg"."PublicSubkeyPacket"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::PublicSubkeyPacket {
    type Target = crate::Org::BouncyCastle::Bcpg::PublicKeyPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::PublicSubkeyPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
impl crate::Org::BouncyCastle::Bcpg::PublicSubkeyPacket {
    pub fn Encode(
        &mut self,
        bcpgOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (bcpgOut))?;
        Ok(__cordl_ret)
    }
    pub fn New_BcpgInputStream0(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyAlgorithmTag_DateTime_IBcpgKey1(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        _cordl_time: crate::System::DateTime,
        key: *mut crate::Org::BouncyCastle::Bcpg::IBcpgKey,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, _cordl_time, key))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyAlgorithmTag_DateTime_IBcpgKey1(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        _cordl_time: crate::System::DateTime,
        key: *mut crate::Org::BouncyCastle::Bcpg::IBcpgKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, _cordl_time, key))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+PublicSubkeyPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::PublicSubkeyPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
