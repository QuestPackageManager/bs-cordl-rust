#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_BakedEventData {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_AssetEventData,
    pub IsNewBake: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_BakedEventData =>
    "HoudiniEngineUnity"."HEU_BakedEventData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_BakedEventData {
    type Target = crate::HoudiniEngineUnity::HEU_AssetEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_BakedEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
impl crate::HoudiniEngineUnity::HEU_BakedEventData {
    pub fn New(
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        successful: bool,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
        isNewBake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asset, successful, outputObjects, isNewBake))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
        successful: bool,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
        isNewBake: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asset, successful, outputObjects, isNewBake))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_BakedEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
