#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerAvatarAudioController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _connectedPlayer: *mut IConnectedPlayer,
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::MultiplayerAvatarAudioController => "BeatSaber.AvatarCore"
    ."MultiplayerAvatarAudioController"
);
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::MultiplayerAvatarAudioController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::MultiplayerAvatarAudioController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
impl crate::BeatSaber::AvatarCore::MultiplayerAvatarAudioController {
    #[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController+_Start_d__4")]
    pub type _Start_d__4 = crate::BeatSaber::AvatarCore::MultiplayerAvatarAudioController__Start_d__4;
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_connectedPlayer(
        &mut self,
        value: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectedPlayer", (value))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarAudioController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::MultiplayerAvatarAudioController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
