#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeBakingOverride_ActiveStateHandling {
    Disable = 2i32,
    Enable = 1i32,
    LeaveAsIs = 0i32,
}
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling => ""
    ."ReflectionProbeBakingOverride/ActiveStateHandling"
);
#[cfg(feature = "ReflectionProbeBakingOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionProbeBakingOverride {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _stateHandling: crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling,
    pub _setPosition: bool,
    pub _localPosition: crate::UnityEngine::Vector3,
    pub _setRotation: bool,
    pub _localRotation: crate::UnityEngine::Vector3,
    pub _setScale: bool,
    pub _localScale: crate::UnityEngine::Vector3,
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ReflectionProbeBakingOverride
    => ""."ReflectionProbeBakingOverride"
);
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl std::ops::Deref for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl std::ops::DerefMut for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl crate::GlobalNamespace::ReflectionProbeBakingOverride {
    #[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
    pub type ActiveStateHandling = crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdateForProbeBaking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateForProbeBaking", ())?;
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
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
