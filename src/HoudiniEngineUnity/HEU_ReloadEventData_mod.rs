#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ReloadEventData {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_AssetEventData,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ReloadEventData =>
    "HoudiniEngineUnity"."HEU_ReloadEventData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ReloadEventData {
    type Target = crate::HoudiniEngineUnity::HEU_AssetEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ReloadEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
impl crate::HoudiniEngineUnity::HEU_ReloadEventData {
    pub fn New(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        successful: bool,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asset, successful, outputObjects))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        successful: bool,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asset, successful, outputObjects))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ReloadEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ReloadEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
