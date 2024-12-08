#[cfg(feature = "EnvironmentShaderWarmup")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentShaderWarmup {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
    pub _mainCamera: *mut MainCamera,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _parentingTransform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "EnvironmentShaderWarmup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentShaderWarmup => ""."EnvironmentShaderWarmup"
);
#[cfg(feature = "EnvironmentShaderWarmup")]
impl std::ops::Deref for EnvironmentShaderWarmup {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentShaderWarmup")]
impl std::ops::DerefMut for EnvironmentShaderWarmup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentShaderWarmup")]
impl EnvironmentShaderWarmup {
    pub const kNumberOfColumns: i32 = 4i32;
    pub const kNumberOfRows: i32 = 4i32;
    #[cfg(feature = "EnvironmentShaderWarmup+_Start_d__6")]
    pub type _Start_d__6 = crate::GlobalNamespace::EnvironmentShaderWarmup__Start_d__6;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "EnvironmentShaderWarmup")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentShaderWarmup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
