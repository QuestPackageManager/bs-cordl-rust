#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PreAssetEventData {
    __cordl_parent: crate::System::Object,
    pub Asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub AssetType: crate::HoudiniEngineUnity::HEU_AssetEventType,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PreAssetEventData =>
    "HoudiniEngineUnity"."HEU_PreAssetEventData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PreAssetEventData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PreAssetEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
impl crate::HoudiniEngineUnity::HEU_PreAssetEventData {
    pub fn New(
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        assetType: crate::HoudiniEngineUnity::HEU_AssetEventType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asset, assetType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        assetType: crate::HoudiniEngineUnity::HEU_AssetEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asset, assetType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PreAssetEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PreAssetEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}