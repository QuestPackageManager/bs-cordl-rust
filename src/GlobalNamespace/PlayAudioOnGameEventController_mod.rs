#[cfg(feature = "PlayAudioOnGameEventController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayAudioOnGameEventController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioClipQueue: *mut crate::GlobalNamespace::AudioClipQueue,
    pub _eventAudioBindings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding,
    >,
}
#[cfg(feature = "PlayAudioOnGameEventController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayAudioOnGameEventController
    => ""."PlayAudioOnGameEventController"
);
#[cfg(feature = "PlayAudioOnGameEventController")]
impl std::ops::Deref for crate::GlobalNamespace::PlayAudioOnGameEventController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayAudioOnGameEventController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayAudioOnGameEventController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayAudioOnGameEventController")]
impl crate::GlobalNamespace::PlayAudioOnGameEventController {
    #[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
    pub type EventAudioBinding = crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
#[cfg(feature = "PlayAudioOnGameEventController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayAudioOnGameEventController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayAudioOnGameEventController_EventAudioBinding {
    __cordl_parent: crate::System::Object,
    pub _signal: *mut crate::GlobalNamespace::Signal,
    pub _delay: f32,
    pub _localizedAudioClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LocalizedAudioClipSO,
    >,
    pub _audioClipQueue: *mut crate::GlobalNamespace::AudioClipQueue,
    pub _randomObjectPicker: *mut crate::GlobalNamespace::RandomObjectPicker_1<
        *mut crate::GlobalNamespace::LocalizedAudioClipSO,
    >,
}
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding => ""
    ."PlayAudioOnGameEventController/EventAudioBinding"
);
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
impl crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding {
    pub fn Deinit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deinit", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        audioClipQueue: *mut crate::GlobalNamespace::AudioClipQueue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (audioClipQueue))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "PlayAudioOnGameEventController+EventAudioBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
