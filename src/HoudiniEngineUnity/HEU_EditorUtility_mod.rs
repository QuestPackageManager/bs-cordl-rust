#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_EditorUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_EditorUtility =>
    "HoudiniEngineUnity"."HEU_EditorUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_EditorUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_EditorUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl crate::HoudiniEngineUnity::HEU_EditorUtility {
    #[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
    pub type HEU_ReplacePrefabOptions = crate::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions;
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_EditorUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_EditorUtility_HEU_ReplacePrefabOptions {
    ConnectToPrefab = 1i32,
    Default = 0i32,
    ReplaceNameBased = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_EditorUtility+HEU_ReplacePrefabOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_EditorUtility_HEU_ReplacePrefabOptions =>
    "HoudiniEngineUnity"."HEU_EditorUtility/HEU_ReplacePrefabOptions"
);
