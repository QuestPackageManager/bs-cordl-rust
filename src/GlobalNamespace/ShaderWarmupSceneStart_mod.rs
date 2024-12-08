#[cfg(feature = "ShaderWarmupSceneStart")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderWarmupSceneStart {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _sceneSetupData: *mut ShaderWarmupSceneSetupData,
    pub _gameScenesManager: *mut GameScenesManager,
}
#[cfg(feature = "ShaderWarmupSceneStart")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ShaderWarmupSceneStart => ""."ShaderWarmupSceneStart"
);
#[cfg(feature = "ShaderWarmupSceneStart")]
impl std::ops::Deref for ShaderWarmupSceneStart {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneStart")]
impl std::ops::DerefMut for ShaderWarmupSceneStart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneStart")]
impl ShaderWarmupSceneStart {
    #[cfg(feature = "ShaderWarmupSceneStart+_Start_d__2")]
    pub type _Start_d__2 = crate::GlobalNamespace::ShaderWarmupSceneStart__Start_d__2;
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ShaderWarmupSceneStart")]
impl quest_hook::libil2cpp::ObjectType for ShaderWarmupSceneStart {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
