#[cfg(feature = "RpcPool")]
#[repr(C)]
#[derive(Debug)]
pub struct RpcPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RpcPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RpcPool => ""."RpcPool"
);
#[cfg(feature = "RpcPool")]
impl std::ops::Deref for crate::GlobalNamespace::RpcPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RpcPool")]
impl std::ops::DerefMut for crate::GlobalNamespace::RpcPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RpcPool")]
impl crate::GlobalNamespace::RpcPool {}
#[cfg(feature = "RpcPool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RpcPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
