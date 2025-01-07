#[cfg(feature = "ObstacleMaterialSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleMaterialSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lwCoreMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _hwCoreMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _texturedCoreMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _fakeGlowLWMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _fakeGlowTexturedMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _obstacleCoreRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    pub _obstacleFakeGlowRenderer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Renderer,
    >,
}
#[cfg(feature = "ObstacleMaterialSetter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ObstacleMaterialSetter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObstacleMaterialSetter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleMaterialSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleMaterialSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl crate::GlobalNamespace::ObstacleMaterialSetter {
    pub fn Init(
        &mut self,
        obstacleQuality: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
        screenDisplacementEffects: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (obstacleQuality, screenDisplacementEffects))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetCoreMaterial(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        obstacleQuality: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
        screenDisplacementEffects: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetCoreMaterial",
                (renderer, obstacleQuality, screenDisplacementEffects),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFakeGlowMaterial(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        obstacleQuality: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFakeGlowMaterial", (renderer, obstacleQuality))?;
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
#[cfg(feature = "ObstacleMaterialSetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleMaterialSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
