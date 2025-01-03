#[cfg(feature = "FlickeringNeonSign")]
#[repr(C)]
#[derive(Debug)]
pub struct FlickeringNeonSign {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _flickeringSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
    pub _light: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TubeBloomPrePassLight>,
    pub _particleSystems: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ParticleSystem>,
    >,
    pub _minOnDelay: f32,
    pub _maxOnDelay: f32,
    pub _minOffDelay: f32,
    pub _maxOffDelay: f32,
    pub _spriteOnColor: crate::UnityEngine::Color,
    pub _lightOnColor: crate::UnityEngine::Color,
    pub _onMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _offMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _sparksAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::AudioClip>,
    >,
    pub _sparksAudioClipPicker: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RandomObjectPicker_1<*mut crate::UnityEngine::AudioClip>,
    >,
}
#[cfg(feature = "FlickeringNeonSign")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlickeringNeonSign => ""
    ."FlickeringNeonSign"
);
#[cfg(feature = "FlickeringNeonSign")]
impl std::ops::Deref for crate::GlobalNamespace::FlickeringNeonSign {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlickeringNeonSign")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlickeringNeonSign {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlickeringNeonSign")]
impl crate::GlobalNamespace::FlickeringNeonSign {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlickeringCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("FlickeringCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "FlickeringNeonSign")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlickeringNeonSign {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
