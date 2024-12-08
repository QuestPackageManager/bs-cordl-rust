#[cfg(feature = "StringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StringExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "StringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StringExtensions => ""."StringExtensions"
);
#[cfg(feature = "StringExtensions")]
impl std::ops::Deref for StringExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StringExtensions")]
impl std::ops::DerefMut for StringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StringExtensions")]
impl StringExtensions {}
#[cfg(feature = "StringExtensions")]
impl quest_hook::libil2cpp::ObjectType for StringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
