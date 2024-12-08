#[cfg(feature = "ModestTree+MiscExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+MiscExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::MiscExtensions => "ModestTree"
    ."MiscExtensions"
);
#[cfg(feature = "ModestTree+MiscExtensions")]
impl std::ops::Deref for crate::ModestTree::MiscExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl std::ops::DerefMut for crate::ModestTree::MiscExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl crate::ModestTree::MiscExtensions {}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::MiscExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
