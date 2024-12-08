#[cfg(feature = "HEU_ScriptParameterExample")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ScriptParameterExample {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _evergreenGameObject: *mut crate::UnityEngine::GameObject,
    pub _evergreenAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub _updateRate: f32,
    pub _scale: f32,
}
#[cfg(feature = "HEU_ScriptParameterExample")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HEU_ScriptParameterExample =>
    ""."HEU_ScriptParameterExample"
);
#[cfg(feature = "HEU_ScriptParameterExample")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ScriptParameterExample {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptParameterExample")]
impl std::ops::DerefMut for crate::GlobalNamespace::HEU_ScriptParameterExample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptParameterExample")]
impl crate::GlobalNamespace::HEU_ScriptParameterExample {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGravity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGravity", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "HEU_ScriptParameterExample")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HEU_ScriptParameterExample {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
