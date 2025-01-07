#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioMixerSnapshot {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Audio::AudioMixerSnapshot {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Audio";
    const CLASS_NAME: &'static str = "AudioMixerSnapshot";
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
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl std::ops::Deref for crate::UnityEngine::Audio::AudioMixerSnapshot {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl std::ops::DerefMut for crate::UnityEngine::Audio::AudioMixerSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl crate::UnityEngine::Audio::AudioMixerSnapshot {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TransitionTo(
        &mut self,
        timeToReach: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionTo", (timeToReach))?;
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
    pub fn get_audioMixer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Audio::AudioMixer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Audio::AudioMixer,
        > = __cordl_object.invoke("get_audioMixer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Audio::AudioMixerSnapshot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl AsRef<crate::UnityEngine::Internal::ISubAssetNotDuplicatable>
for crate::UnityEngine::Audio::AudioMixerSnapshot {
    fn as_ref(&self) -> &crate::UnityEngine::Internal::ISubAssetNotDuplicatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
impl AsMut<crate::UnityEngine::Internal::ISubAssetNotDuplicatable>
for crate::UnityEngine::Audio::AudioMixerSnapshot {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Internal::ISubAssetNotDuplicatable {
        unsafe { std::mem::transmute(self) }
    }
}
