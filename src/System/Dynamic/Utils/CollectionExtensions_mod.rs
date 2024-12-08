#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::CollectionExtensions =>
    "System.Dynamic.Utils"."CollectionExtensions"
);
#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
impl std::ops::Deref for crate::System::Dynamic::Utils::CollectionExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::CollectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
impl crate::System::Dynamic::Utils::CollectionExtensions {}
#[cfg(feature = "System+Dynamic+Utils+CollectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::CollectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
