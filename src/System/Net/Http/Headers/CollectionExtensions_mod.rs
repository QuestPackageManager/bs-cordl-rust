#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::CollectionExtensions
    => "System.Net.Http.Headers"."CollectionExtensions"
);
#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
impl std::ops::Deref for crate::System::Net::Http::Headers::CollectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::CollectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
impl crate::System::Net::Http::Headers::CollectionExtensions {}
#[cfg(feature = "System+Net+Http+Headers+CollectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::CollectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
