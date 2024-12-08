#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct HashCodeCalculator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HashCodeCalculator
    => "System.Net.Http.Headers"."HashCodeCalculator"
);
#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HashCodeCalculator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HashCodeCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
impl crate::System::Net::Http::Headers::HashCodeCalculator {}
#[cfg(feature = "System+Net+Http+Headers+HashCodeCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HashCodeCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
