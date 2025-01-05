#[cfg(feature = "MenuNeonLightsGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuNeonLightsGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _generate: bool,
    pub _radius: f32,
    pub _angle: f32,
    pub _numberOfElements: i32,
    pub _intensityCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _intensityMultiplier: f32,
    pub _lengthCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _lengthMultiplier: f32,
    pub _widthCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _widthMultiplier: f32,
    pub _neonLightPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _afterSpawnRotation: crate::UnityEngine::Vector3,
}
#[cfg(feature = "MenuNeonLightsGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuNeonLightsGenerator => ""
    ."MenuNeonLightsGenerator"
);
#[cfg(feature = "MenuNeonLightsGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::MenuNeonLightsGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuNeonLightsGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuNeonLightsGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuNeonLightsGenerator")]
impl crate::GlobalNamespace::MenuNeonLightsGenerator {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "MenuNeonLightsGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuNeonLightsGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
