#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PluginSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PluginSettings =>
    "HoudiniEngineUnity"."HEU_PluginSettings"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PluginSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PluginSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl crate::HoudiniEngineUnity::HEU_PluginSettings {}
#[cfg(feature = "HoudiniEngineUnity+HEU_PluginSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PluginSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
