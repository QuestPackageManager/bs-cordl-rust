#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioMixer {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Audio::AudioMixer =>
    "UnityEngine.Audio"."AudioMixer"
);
#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
impl std::ops::Deref for crate::UnityEngine::Audio::AudioMixer {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
impl std::ops::DerefMut for crate::UnityEngine::Audio::AudioMixer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
impl crate::UnityEngine::Audio::AudioMixer {
    pub fn ClearFloat(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ClearFloat", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMatchingGroups(
        &mut self,
        subPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Audio::AudioMixerGroup,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Audio::AudioMixerGroup,
            >,
        > = __cordl_object.invoke("FindMatchingGroups", (subPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindSnapshot(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Audio::AudioMixerSnapshot>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Audio::AudioMixerSnapshot,
        > = __cordl_object.invoke("FindSnapshot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbsoluteAudibilityFromGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<crate::UnityEngine::Audio::AudioMixerGroup>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetAbsoluteAudibilityFromGroup", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloat(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetFloat", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetFloat(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetFloat", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToSnapshot(
        &mut self,
        snapshot: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Audio::AudioMixerSnapshot,
        >,
        timeToReach: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToSnapshot", (snapshot, timeToReach))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToSnapshotInternal(
        &mut self,
        snapshot: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Audio::AudioMixerSnapshot,
        >,
        timeToReach: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToSnapshotInternal", (snapshot, timeToReach))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToSnapshots(
        &mut self,
        snapshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::Audio::AudioMixerSnapshot,
            >,
        >,
        weights: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        timeToReach: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToSnapshots", (snapshots, weights, timeToReach))?;
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
    pub fn get_outputAudioMixerGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Audio::AudioMixerGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Audio::AudioMixerGroup,
        > = __cordl_object.invoke("get_outputAudioMixerGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Audio::AudioMixerUpdateMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Audio::AudioMixerUpdateMode = __cordl_object
            .invoke("get_updateMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_outputAudioMixerGroup(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Audio::AudioMixerGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_outputAudioMixerGroup", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_updateMode(
        &mut self,
        value: crate::UnityEngine::Audio::AudioMixerUpdateMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Audio+AudioMixer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Audio::AudioMixer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
