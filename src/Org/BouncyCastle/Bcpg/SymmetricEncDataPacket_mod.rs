#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct SymmetricEncDataPacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::InputStreamPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::SymmetricEncDataPacket
    => "Org.BouncyCastle.Bcpg"."SymmetricEncDataPacket"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::SymmetricEncDataPacket {
    type Target = crate::Org::BouncyCastle::Bcpg::InputStreamPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::SymmetricEncDataPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
impl crate::Org::BouncyCastle::Bcpg::SymmetricEncDataPacket {
    pub fn New(
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SymmetricEncDataPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::SymmetricEncDataPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
