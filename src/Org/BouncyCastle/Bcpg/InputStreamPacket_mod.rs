#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct InputStreamPacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::Packet,
    pub bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::InputStreamPacket =>
    "Org.BouncyCastle.Bcpg"."InputStreamPacket"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::InputStreamPacket {
    type Target = crate::Org::BouncyCastle::Bcpg::Packet;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::InputStreamPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
impl crate::Org::BouncyCastle::Bcpg::InputStreamPacket {
    pub fn GetInputStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream = __cordl_object
            .invoke("GetInputStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
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
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+InputStreamPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::InputStreamPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
