#[cfg(feature = "StaticEnvironmentLights")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticEnvironmentLights {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightColors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    pub _materials: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Material,
    >,
}
#[cfg(feature = "StaticEnvironmentLights")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StaticEnvironmentLights => ""."StaticEnvironmentLights"
);
#[cfg(feature = "StaticEnvironmentLights")]
impl std::ops::Deref for StaticEnvironmentLights {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticEnvironmentLights")]
impl std::ops::DerefMut for StaticEnvironmentLights {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticEnvironmentLights")]
impl StaticEnvironmentLights {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "StaticEnvironmentLights")]
impl quest_hook::libil2cpp::ObjectType for StaticEnvironmentLights {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
