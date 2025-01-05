#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_CookedEventData {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_AssetEventData,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_CookedEventData =>
    "HoudiniEngineUnity"."HEU_CookedEventData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_CookedEventData {
    type Target = quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_AssetEventData,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_CookedEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
impl crate::HoudiniEngineUnity::HEU_CookedEventData {
    pub fn New(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        successful: bool,
        outputObjects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_CookedEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
