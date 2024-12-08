#[cfg(feature = "SetTubeBloomPrePassLightColor")]
#[repr(C)]
#[derive(Debug)]
pub struct SetTubeBloomPrePassLightColor {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _color: *mut crate::GlobalNamespace::ColorSO,
    pub _tubeLights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
}
#[cfg(feature = "SetTubeBloomPrePassLightColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SetTubeBloomPrePassLightColor
    => ""."SetTubeBloomPrePassLightColor"
);
#[cfg(feature = "SetTubeBloomPrePassLightColor")]
impl std::ops::Deref for crate::GlobalNamespace::SetTubeBloomPrePassLightColor {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SetTubeBloomPrePassLightColor")]
impl std::ops::DerefMut for crate::GlobalNamespace::SetTubeBloomPrePassLightColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SetTubeBloomPrePassLightColor")]
impl crate::GlobalNamespace::SetTubeBloomPrePassLightColor {
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
#[cfg(feature = "SetTubeBloomPrePassLightColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SetTubeBloomPrePassLightColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
