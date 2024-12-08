#[cfg(feature = "StringBuilderExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilderExtension {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "StringBuilderExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StringBuilderExtension => ""."StringBuilderExtension"
);
#[cfg(feature = "StringBuilderExtension")]
impl std::ops::Deref for StringBuilderExtension {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StringBuilderExtension")]
impl std::ops::DerefMut for StringBuilderExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StringBuilderExtension")]
impl StringBuilderExtension {
    pub const kCharZero: char = '0';
}
#[cfg(feature = "StringBuilderExtension")]
impl quest_hook::libil2cpp::ObjectType for StringBuilderExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
