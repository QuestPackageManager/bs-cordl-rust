#[cfg(feature = "GraphicSettingHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicSettingHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "GraphicSettingHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GraphicSettingHelper => ""."GraphicSettingHelper"
);
#[cfg(feature = "GraphicSettingHelper")]
impl std::ops::Deref for GraphicSettingHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GraphicSettingHelper")]
impl std::ops::DerefMut for GraphicSettingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GraphicSettingHelper")]
impl GraphicSettingHelper {}
#[cfg(feature = "GraphicSettingHelper")]
impl quest_hook::libil2cpp::ObjectType for GraphicSettingHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
