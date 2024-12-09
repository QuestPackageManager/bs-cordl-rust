#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::CollectionParser =>
    "System.Net.Http.Headers"."CollectionParser"
);
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl std::ops::Deref for crate::System::Net::Http::Headers::CollectionParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::CollectionParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl crate::System::Net::Http::Headers::CollectionParser {}
#[cfg(feature = "System+Net+Http+Headers+CollectionParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::CollectionParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
