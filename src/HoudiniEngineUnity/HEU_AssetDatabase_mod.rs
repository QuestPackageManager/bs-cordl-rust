#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetDatabase {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetDatabase =>
    "HoudiniEngineUnity"."HEU_AssetDatabase"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl crate::HoudiniEngineUnity::HEU_AssetDatabase {
    #[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
    pub type HEU_ImportAssetOptions = crate::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions;
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_AssetDatabase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_AssetDatabase_HEU_ImportAssetOptions {
    Default = 0i32,
    DontDownloadFromCacheServer = 8192i32,
    ForceSynchronousImport = 8i32,
    ForceUncompressedImport = 16384i32,
    ForceUpdate = 1i32,
    ImportRecursive = 256i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetDatabase+HEU_ImportAssetOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_AssetDatabase_HEU_ImportAssetOptions =>
    "HoudiniEngineUnity"."HEU_AssetDatabase/HEU_ImportAssetOptions"
);
