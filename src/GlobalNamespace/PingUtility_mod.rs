#[cfg(feature = "PingUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct PingUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PingUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PingUtility => ""."PingUtility"
);
#[cfg(feature = "PingUtility")]
impl std::ops::Deref for crate::GlobalNamespace::PingUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PingUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::PingUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PingUtility")]
impl crate::GlobalNamespace::PingUtility {
    #[cfg(feature = "PingUtility+_PingAsync_d__0")]
    pub type _PingAsync_d__0 = crate::GlobalNamespace::PingUtility__PingAsync_d__0;
    #[cfg(feature = "PingUtility+__c")]
    pub type __c = crate::GlobalNamespace::PingUtility___c;
}
#[cfg(feature = "PingUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PingUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
