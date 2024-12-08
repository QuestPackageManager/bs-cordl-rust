#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct ShowTextOnGameEventController_EventTextBinding {
    __cordl_parent: crate::System::Object,
    pub _signal: *mut Signal,
    pub _text: *mut crate::System::String,
    pub _textFadeTransitions: *mut TextFadeTransitions,
}
#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding => ""
    ."ShowTextOnGameEventController/EventTextBinding"
);
#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
impl std::ops::Deref
for crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
impl crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding {
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
        textFadeTransitions: *mut TextFadeTransitions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (textFadeTransitions))?;
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
#[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ShowTextOnGameEventController")]
#[repr(C)]
#[derive(Debug)]
pub struct ShowTextOnGameEventController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _textFadeTransitions: *mut TextFadeTransitions,
    pub _eventTextBindings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding,
    >,
}
#[cfg(feature = "ShowTextOnGameEventController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ShowTextOnGameEventController => ""
    ."ShowTextOnGameEventController"
);
#[cfg(feature = "ShowTextOnGameEventController")]
impl std::ops::Deref for ShowTextOnGameEventController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShowTextOnGameEventController")]
impl std::ops::DerefMut for ShowTextOnGameEventController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShowTextOnGameEventController")]
impl ShowTextOnGameEventController {
    #[cfg(feature = "ShowTextOnGameEventController+EventTextBinding")]
    pub type EventTextBinding = crate::GlobalNamespace::ShowTextOnGameEventController_EventTextBinding;
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
#[cfg(feature = "ShowTextOnGameEventController")]
impl quest_hook::libil2cpp::ObjectType for ShowTextOnGameEventController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}