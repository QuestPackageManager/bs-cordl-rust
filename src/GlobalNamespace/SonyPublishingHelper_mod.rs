#[cfg(feature = "SonyPublishingHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyPublishingHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SonyPublishingHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SonyPublishingHelper => ""."SonyPublishingHelper"
);
#[cfg(feature = "SonyPublishingHelper")]
impl std::ops::Deref for SonyPublishingHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublishingHelper")]
impl std::ops::DerefMut for SonyPublishingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPublishingHelper")]
impl SonyPublishingHelper {}
#[cfg(feature = "SonyPublishingHelper")]
impl quest_hook::libil2cpp::ObjectType for SonyPublishingHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}