#[cfg(feature = "NetworkUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "NetworkUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkUtility => ""
    ."NetworkUtility"
);
#[cfg(feature = "NetworkUtility")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkUtility")]
impl crate::GlobalNamespace::NetworkUtility {}
#[cfg(feature = "NetworkUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NetworkUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
