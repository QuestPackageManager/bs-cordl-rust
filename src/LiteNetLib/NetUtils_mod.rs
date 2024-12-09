#[cfg(feature = "LiteNetLib+NetUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct NetUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+NetUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetUtils => "LiteNetLib"."NetUtils"
);
#[cfg(feature = "LiteNetLib+NetUtils")]
impl std::ops::Deref for crate::LiteNetLib::NetUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl std::ops::DerefMut for crate::LiteNetLib::NetUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl crate::LiteNetLib::NetUtils {}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
