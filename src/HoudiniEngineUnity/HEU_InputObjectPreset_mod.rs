#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputObjectPreset {
    __cordl_parent: crate::System::Object,
    pub _gameObjectName: *mut crate::System::String,
    pub _isSceneObject: bool,
    pub _useTransformOffset: bool,
    pub _translateOffset: crate::UnityEngine::Vector3,
    pub _rotateOffset: crate::UnityEngine::Vector3,
    pub _scaleOffset: crate::UnityEngine::Vector3,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputObjectPreset =>
    "HoudiniEngineUnity"."HEU_InputObjectPreset"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputObjectPreset {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputObjectPreset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
impl crate::HoudiniEngineUnity::HEU_InputObjectPreset {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectPreset")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputObjectPreset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
