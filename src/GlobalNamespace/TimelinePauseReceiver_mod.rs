#[cfg(feature = "TimelinePauseReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelinePauseReceiver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub timelinePauseEvent: *mut crate::System::Action,
}
#[cfg(feature = "TimelinePauseReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimelinePauseReceiver => ""
    ."TimelinePauseReceiver"
);
#[cfg(feature = "TimelinePauseReceiver")]
impl std::ops::Deref for crate::GlobalNamespace::TimelinePauseReceiver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelinePauseReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl crate::GlobalNamespace::TimelinePauseReceiver {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnNotify(
        &mut self,
        origin: crate::UnityEngine::Playables::Playable,
        notification: *mut crate::UnityEngine::Playables::INotification,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNotify", (origin, notification, context))?;
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
    pub fn add_timelinePauseEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_timelinePauseEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_timelinePauseEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_timelinePauseEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TimelinePauseReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
