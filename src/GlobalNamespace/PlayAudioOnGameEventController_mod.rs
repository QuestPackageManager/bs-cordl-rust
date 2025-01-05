#[cfg(feature = "PlayAudioOnGameEventController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayAudioOnGameEventController {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _audioClipQueue: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipQueue,
    >,
    pub _eventAudioBindings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PlayAudioOnGameEventController_EventAudioBinding,
            >,
        >,
    >,
}
#[cfg(feature = "PlayAudioOnGameEventController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayAudioOnGameEventController
    => ""."PlayAudioOnGameEventController"
);
#[cfg(feature = "PlayAudioOnGameEventController")]
impl std::ops::Deref for crate::GlobalNamespace::PlayAudioOnGameEventController {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _signal: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Signal>,
    pub _delay: f32,
    pub _localizedAudioClips: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalizedAudioClipSO>,
        >,
    >,
    pub _audioClipQueue: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipQueue,
    >,
    pub _randomObjectPicker: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalizedAudioClipSO>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        audioClipQueue: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioClipQueue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (audioClipQueue))?;
        Ok(__cordl_ret.into())
    }
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
