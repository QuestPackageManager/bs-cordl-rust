#[cfg(feature = "ReflectionProbeDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionProbeDataSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _reflectionProbeCubemap1: *mut crate::UnityEngine::Cubemap,
    pub _reflectionProbeCubemap2: *mut crate::UnityEngine::Cubemap,
}
#[cfg(feature = "ReflectionProbeDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ReflectionProbeDataSO => ""."ReflectionProbeDataSO"
);
#[cfg(feature = "ReflectionProbeDataSO")]
impl std::ops::Deref for ReflectionProbeDataSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeDataSO")]
impl std::ops::DerefMut for ReflectionProbeDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeDataSO")]
impl ReflectionProbeDataSO {
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
    pub fn get_reflectionProbeCubemap1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Cubemap> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Cubemap = __cordl_object
            .invoke("get_reflectionProbeCubemap1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_reflectionProbeCubemap2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Cubemap> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Cubemap = __cordl_object
            .invoke("get_reflectionProbeCubemap2", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_reflectionProbeCubemap1(
        &mut self,
        value: *mut crate::UnityEngine::Cubemap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reflectionProbeCubemap1", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_reflectionProbeCubemap2(
        &mut self,
        value: *mut crate::UnityEngine::Cubemap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reflectionProbeCubemap2", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ReflectionProbeDataSO")]
impl quest_hook::libil2cpp::ObjectType for ReflectionProbeDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
