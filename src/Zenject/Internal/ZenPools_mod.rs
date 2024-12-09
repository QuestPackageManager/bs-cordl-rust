#[cfg(feature = "Zenject+Internal+ZenPools")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenPools {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ZenPools =>
    "Zenject.Internal"."ZenPools"
);
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl std::ops::Deref for crate::Zenject::Internal::ZenPools {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl std::ops::DerefMut for crate::Zenject::Internal::ZenPools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl crate::Zenject::Internal::ZenPools {}
#[cfg(feature = "Zenject+Internal+ZenPools")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ZenPools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
