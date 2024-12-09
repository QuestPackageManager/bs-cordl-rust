#[cfg(feature = "System+Text+StringBuilderCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilderCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+StringBuilderCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::StringBuilderCache =>
    "System.Text"."StringBuilderCache"
);
#[cfg(feature = "System+Text+StringBuilderCache")]
impl std::ops::Deref for crate::System::Text::StringBuilderCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl std::ops::DerefMut for crate::System::Text::StringBuilderCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl crate::System::Text::StringBuilderCache {}
#[cfg(feature = "System+Text+StringBuilderCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::StringBuilderCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
