#[cfg(feature = "System+Text+EncodingHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+EncodingHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::EncodingHelper => "System.Text"
    ."EncodingHelper"
);
#[cfg(feature = "System+Text+EncodingHelper")]
impl std::ops::Deref for crate::System::Text::EncodingHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingHelper")]
impl std::ops::DerefMut for crate::System::Text::EncodingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingHelper")]
impl crate::System::Text::EncodingHelper {}
#[cfg(feature = "System+Text+EncodingHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::EncodingHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
