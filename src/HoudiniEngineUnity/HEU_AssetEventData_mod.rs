#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_AssetEventData {
    __cordl_parent: crate::System::Object,
    pub Asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub CookSuccess: bool,
    pub OutputObjects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub EventType: crate::HoudiniEngineUnity::HEU_AssetEventType,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_AssetEventData =>
    "HoudiniEngineUnity"."HEU_AssetEventData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_AssetEventData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_AssetEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
impl crate::HoudiniEngineUnity::HEU_AssetEventData {
    pub fn _ctor(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        successful: bool,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asset, successful, outputObjects))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        successful: bool,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asset, successful, outputObjects))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_AssetEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_AssetEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
