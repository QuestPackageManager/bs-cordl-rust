#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioMixerGroup {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioMixerGroup =>
    "UnityEngine.Audio"."AudioMixerGroup"
);
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl std::ops::Deref for crate::UnityEngine::Audio::AudioMixerGroup {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl std::ops::DerefMut for crate::UnityEngine::Audio::AudioMixerGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl crate::UnityEngine::Audio::AudioMixerGroup {
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
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Audio::AudioMixerGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl AsRef<crate::UnityEngine::Internal::ISubAssetNotDuplicatable>
for crate::UnityEngine::Audio::AudioMixerGroup {
    fn as_ref(&self) -> &crate::UnityEngine::Internal::ISubAssetNotDuplicatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixerGroup")]
impl AsMut<crate::UnityEngine::Internal::ISubAssetNotDuplicatable>
for crate::UnityEngine::Audio::AudioMixerGroup {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Internal::ISubAssetNotDuplicatable {
        unsafe { std::mem::transmute(self) }
    }
}
