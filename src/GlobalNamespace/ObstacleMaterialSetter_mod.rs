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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleMaterialSetter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Init", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obstacleQuality, screenDisplacementEffects))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                            crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetCoreMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetCoreMaterial", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (renderer, obstacleQuality, screenDisplacementEffects),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFakeGlowMaterial(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        obstacleQuality: crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                            crate::BeatSaber::Settings::QualitySettings_ObstacleQuality,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetFakeGlowMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetFakeGlowMaterial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderer, obstacleQuality))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
