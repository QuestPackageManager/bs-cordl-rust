#[cfg(feature = "SaberTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberTypeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SaberTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SaberTypeExtensions => ""."SaberTypeExtensions"
);
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::Deref for SaberTypeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl std::ops::DerefMut for SaberTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberTypeExtensions")]
impl SaberTypeExtensions {}
#[cfg(feature = "SaberTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for SaberTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}