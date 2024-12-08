#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
#[repr(C)]
#[derive(Debug)]
pub struct ActivationControlPlayable {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub gameObject: *mut crate::UnityEngine::GameObject,
    pub postPlayback: crate::UnityEngine::Timeline::ActivationControlPlayable_PostPlaybackState,
    pub m_InitialState: crate::UnityEngine::Timeline::ActivationControlPlayable_InitialState,
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::ActivationControlPlayable
    => "UnityEngine.Timeline"."ActivationControlPlayable"
);
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
impl std::ops::Deref for crate::UnityEngine::Timeline::ActivationControlPlayable {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::ActivationControlPlayable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
impl crate::UnityEngine::Timeline::ActivationControlPlayable {
    #[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+PostPlaybackState")]
    pub type PostPlaybackState = crate::UnityEngine::Timeline::ActivationControlPlayable_PostPlaybackState;
    #[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+InitialState")]
    pub type InitialState = crate::UnityEngine::Timeline::ActivationControlPlayable_InitialState;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnBehaviourPause(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBehaviourPause", (playable, info))?;
        Ok(__cordl_ret)
    }
    pub fn OnBehaviourPlay(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBehaviourPlay", (playable, info))?;
        Ok(__cordl_ret)
    }
    pub fn OnGraphStart(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGraphStart", (playable))?;
        Ok(__cordl_ret)
    }
    pub fn OnPlayableDestroy(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPlayableDestroy", (playable))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFrame", (playable, info, userData))?;
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
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::ActivationControlPlayable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+InitialState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationControlPlayable_InitialState {
    Active = 1i32,
    Inactive = 2i32,
    Unset = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+InitialState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::ActivationControlPlayable_InitialState =>
    "UnityEngine.Timeline"."ActivationControlPlayable/InitialState"
);
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+PostPlaybackState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationControlPlayable_PostPlaybackState {
    Active = 0i32,
    Inactive = 1i32,
    Revert = 2i32,
}
#[cfg(feature = "UnityEngine+Timeline+ActivationControlPlayable+PostPlaybackState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::ActivationControlPlayable_PostPlaybackState =>
    "UnityEngine.Timeline"."ActivationControlPlayable/PostPlaybackState"
);
