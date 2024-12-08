#[cfg(feature = "ColorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ColorExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ColorExtensions => ""."ColorExtensions"
);
#[cfg(feature = "ColorExtensions")]
impl std::ops::Deref for ColorExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorExtensions")]
impl std::ops::DerefMut for ColorExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorExtensions")]
impl ColorExtensions {}
#[cfg(feature = "ColorExtensions")]
impl quest_hook::libil2cpp::ObjectType for ColorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
