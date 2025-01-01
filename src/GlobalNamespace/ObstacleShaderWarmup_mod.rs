#[cfg(feature = "ObstacleShaderWarmup")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleShaderWarmup {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _obstacleMaterialSetter: *mut crate::GlobalNamespace::ObstacleMaterialSetter,
    pub _settingsManager: *mut crate::GlobalNamespace::SettingsManager,
}
#[cfg(feature = "ObstacleShaderWarmup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleShaderWarmup => ""
    ."ObstacleShaderWarmup"
);
#[cfg(feature = "ObstacleShaderWarmup")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleShaderWarmup {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleShaderWarmup")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleShaderWarmup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleShaderWarmup")]
impl crate::GlobalNamespace::ObstacleShaderWarmup {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObstacleShaderWarmup")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ObstacleShaderWarmup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}