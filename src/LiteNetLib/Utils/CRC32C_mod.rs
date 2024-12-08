#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
#[repr(C)]
#[derive(Debug)]
pub struct CRC32C {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::CRC32C => "LiteNetLib.Utils"
    ."CRC32C"
);
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl std::ops::Deref for crate::LiteNetLib::Utils::CRC32C {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::CRC32C {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl crate::LiteNetLib::Utils::CRC32C {
    pub const ChecksumSize: i32 = 4i32;
    pub const Poly: u32 = 4131092720u32;
}
#[cfg(feature = "LiteNetLib+Utils+CRC32C")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::CRC32C {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
