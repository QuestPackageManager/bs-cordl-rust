#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
#[repr(C)]
#[derive(Debug)]
pub struct ActivationTrack {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>,
    pub m_PostPlaybackState: crate::UnityEngine::Timeline::ActivationTrack_PostPlaybackState,
    pub m_ActivationMixer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::ActivationMixerPlayable,
    >,
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::ActivationTrack =>
    "UnityEngine.Timeline"."ActivationTrack"
);
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
impl std::ops::Deref for crate::UnityEngine::Timeline::ActivationTrack {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TrackAsset>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::ActivationTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
impl crate::UnityEngine::Timeline::ActivationTrack {
    #[cfg(feature = "UnityEngine+Timeline+ActivationTrack+PostPlaybackState")]
    pub type PostPlaybackState = crate::UnityEngine::Timeline::ActivationTrack_PostPlaybackState;
    pub fn CanCompileClips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCompileClips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTrackMixer(
        &mut self,
        graph: crate::UnityEngine::Playables::PlayableGraph,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::Playable = __cordl_object
            .invoke("CreateTrackMixer", (graph, go, inputCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherProperties(
        &mut self,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
        driver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::IPropertyCollector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherProperties", (director, driver))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCreateClip(
        &mut self,
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::TimelineClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCreateClip", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTrackMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTrackMode", ())?;
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
    pub fn get_postPlaybackState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::ActivationTrack_PostPlaybackState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::ActivationTrack_PostPlaybackState = __cordl_object
            .invoke("get_postPlaybackState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_postPlaybackState(
        &mut self,
        value: crate::UnityEngine::Timeline::ActivationTrack_PostPlaybackState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_postPlaybackState", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::ActivationTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack+PostPlaybackState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivationTrack_PostPlaybackState {
    #[default]
    Active = 0i32,
    Inactive = 1i32,
    LeaveAsIs = 3i32,
    Revert = 2i32,
}
#[cfg(feature = "UnityEngine+Timeline+ActivationTrack+PostPlaybackState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::ActivationTrack_PostPlaybackState => "UnityEngine.Timeline"
    ."ActivationTrack/PostPlaybackState"
);
