#[cfg(feature = "FlickeringNeonSign")]
#[repr(C)]
#[derive(Debug)]
pub struct FlickeringNeonSign {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flickeringSprite: *mut crate::UnityEngine::SpriteRenderer,
    pub _light: *mut TubeBloomPrePassLight,
    pub _particleSystems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::ParticleSystem,
    >,
    pub _minOnDelay: f32,
    pub _maxOnDelay: f32,
    pub _minOffDelay: f32,
    pub _maxOffDelay: f32,
    pub _spriteOnColor: crate::UnityEngine::Color,
    pub _lightOnColor: crate::UnityEngine::Color,
    pub _onMaterial: *mut crate::UnityEngine::Material,
    pub _offMaterial: *mut crate::UnityEngine::Material,
    pub _sparksAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _sparksAudioClipPicker: *mut RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
}
#[cfg(feature = "FlickeringNeonSign")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FlickeringNeonSign => ""."FlickeringNeonSign"
);
#[cfg(feature = "FlickeringNeonSign")]
impl std::ops::Deref for FlickeringNeonSign {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlickeringNeonSign")]
impl std::ops::DerefMut for FlickeringNeonSign {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlickeringNeonSign")]
impl FlickeringNeonSign {
    #[cfg(feature = "FlickeringNeonSign+_FlickeringCoroutine_d__16")]
    pub type _FlickeringCoroutine_d__16 = crate::GlobalNamespace::FlickeringNeonSign__FlickeringCoroutine_d__16;
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
    pub fn FlickeringCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("FlickeringCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetOn(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOn", (on))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "FlickeringNeonSign")]
impl quest_hook::libil2cpp::ObjectType for FlickeringNeonSign {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
