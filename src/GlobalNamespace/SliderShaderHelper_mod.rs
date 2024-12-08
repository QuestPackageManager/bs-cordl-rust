#[cfg(feature = "SliderShaderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderShaderHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SliderShaderHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderShaderHelper => ""."SliderShaderHelper"
);
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::Deref for SliderShaderHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl std::ops::DerefMut for SliderShaderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderShaderHelper")]
impl SliderShaderHelper {}
#[cfg(feature = "SliderShaderHelper")]
impl quest_hook::libil2cpp::ObjectType for SliderShaderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
