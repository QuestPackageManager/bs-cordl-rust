#[cfg(feature = "SliderMidAnchorModeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMidAnchorModeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderMidAnchorModeExtensions => ""
    ."SliderMidAnchorModeExtensions"
);
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl std::ops::Deref for SliderMidAnchorModeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl std::ops::DerefMut for SliderMidAnchorModeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl SliderMidAnchorModeExtensions {}
#[cfg(feature = "SliderMidAnchorModeExtensions")]
impl quest_hook::libil2cpp::ObjectType for SliderMidAnchorModeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
