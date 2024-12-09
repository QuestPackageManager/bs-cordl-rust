#[cfg(feature = "ColorSchemeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ColorSchemeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeConverter => ""
    ."ColorSchemeConverter"
);
#[cfg(feature = "ColorSchemeConverter")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeConverter")]
impl crate::GlobalNamespace::ColorSchemeConverter {}
#[cfg(feature = "ColorSchemeConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorSchemeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
