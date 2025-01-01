#[cfg(feature = "ColorSchemeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ColorSchemeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeExtensions => ""
    ."ColorSchemeExtensions"
);
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeExtensions")]
impl crate::GlobalNamespace::ColorSchemeExtensions {}
#[cfg(feature = "ColorSchemeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorSchemeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}