#[cfg(feature = "ToneMappingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ToneMappingExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ToneMappingExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ToneMappingExtensions => ""."ToneMappingExtensions"
);
#[cfg(feature = "ToneMappingExtensions")]
impl std::ops::Deref for ToneMappingExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ToneMappingExtensions")]
impl std::ops::DerefMut for ToneMappingExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ToneMappingExtensions")]
impl ToneMappingExtensions {}
#[cfg(feature = "ToneMappingExtensions")]
impl quest_hook::libil2cpp::ObjectType for ToneMappingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
