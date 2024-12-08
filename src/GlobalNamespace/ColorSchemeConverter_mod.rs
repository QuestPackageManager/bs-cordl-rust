#[cfg(feature = "ColorSchemeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ColorSchemeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ColorSchemeConverter => ""."ColorSchemeConverter"
);
#[cfg(feature = "ColorSchemeConverter")]
impl std::ops::Deref for ColorSchemeConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeConverter")]
impl std::ops::DerefMut for ColorSchemeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeConverter")]
impl ColorSchemeConverter {}
#[cfg(feature = "ColorSchemeConverter")]
impl quest_hook::libil2cpp::ObjectType for ColorSchemeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
