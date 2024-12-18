#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioMixerSnapshot {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerSnapshot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioMixerSnapshot =>
    "UnityEngine.Audio"."AudioMixerSnapshot"
);
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
