#[cfg(feature = "FireworkItemController")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _particleSystems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem,
    >,
    pub _lights: *mut quest_hook::libil2cpp::Il2CppArray<*mut TubeBloomPrePassLight>,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _lightFlashDuration: f32,
    pub _lightIntensityCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _lightIntensityMultiplier: f32,
    pub _randomizeColor: bool,
    pub _lightsColor: crate::UnityEngine::Color,
    pub _lightsColorGradient: *mut crate::UnityEngine::Gradient,
    pub _randomizeSpeed: bool,
    pub _minSpeedMultiplier: f32,
    pub _maxSpeedMultiplier: f32,
    pub _explosionClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _randomAudioPicker: *mut RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _directionalLight: *mut DirectionalLight,
    pub _directionalLightIntensity: f32,
    pub _initialized: bool,
    pub didFinishEvent: *mut crate::System::Action_1<*mut FireworkItemController>,
}
#[cfg(feature = "FireworkItemController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FireworkItemController => ""."FireworkItemController"
);
#[cfg(feature = "FireworkItemController")]
impl std::ops::Deref for FireworkItemController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController")]
impl std::ops::DerefMut for FireworkItemController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController")]
impl FireworkItemController {
    #[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
    pub type FireworkItemParticleSystem = crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem;
    #[cfg(feature = "FireworkItemController+Pool")]
    pub type Pool = crate::GlobalNamespace::FireworkItemController_Pool;
    #[cfg(feature = "FireworkItemController+_FireCoroutine_d__29")]
    pub type _FireCoroutine_d__29 = crate::GlobalNamespace::FireworkItemController__FireCoroutine_d__29;
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
    pub fn Fire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", ())?;
        Ok(__cordl_ret)
    }
    pub fn FireCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("FireCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeParticleSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeParticleSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn PlayExplosionSound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayExplosionSound", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLightsColor(
        &mut self,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLightsColor", (intensity))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut FireworkItemController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut FireworkItemController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_directionalLight(
        &mut self,
        value: *mut DirectionalLight,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_directionalLight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_directionalLightIntensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_directionalLightIntensity", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FireworkItemController")]
impl quest_hook::libil2cpp::ObjectType for FireworkItemController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController_FireworkItemParticleSystem {
    __cordl_parent: crate::System::Object,
    pub _particleSystem: *mut crate::UnityEngine::ParticleSystem,
    pub _isSubemitter: bool,
    pub _useMainColor: bool,
    pub _useOwnGradient: bool,
    pub _particleColorGradient: *mut crate::UnityEngine::Gradient,
    pub _randomizeSpeed: bool,
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem => ""
    ."FireworkItemController/FireworkItemParticleSystem"
);
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl std::ops::Deref
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
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
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut FireworkItemController>,
}
#[cfg(feature = "FireworkItemController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FireworkItemController_Pool =>
    ""."FireworkItemController/Pool"
);
#[cfg(feature = "FireworkItemController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FireworkItemController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut FireworkItemController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::FireworkItemController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
impl crate::GlobalNamespace::FireworkItemController_Pool {
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
#[cfg(feature = "FireworkItemController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FireworkItemController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
