#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeNotificationBehaviour {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub m_Notifications: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        >,
    >,
    pub m_PreviousTime: f64,
    pub m_NeedSortNotifications: bool,
    pub m_TimeSource: crate::UnityEngine::Playables::Playable,
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimeNotificationBehaviour
    => "UnityEngine.Timeline"."TimeNotificationBehaviour"
);
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimeNotificationBehaviour {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimeNotificationBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
impl crate::UnityEngine::Timeline::TimeNotificationBehaviour {
    #[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
    pub type NotificationEntry = crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry;
    pub fn AddNotification(
        &mut self,
        _cordl_time: f64,
        payload: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification>,
        flags: crate::UnityEngine::Timeline::NotificationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNotification", (_cordl_time, payload, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanRestoreNotification(
        e: crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        info: crate::UnityEngine::Playables::FrameData,
        currentTime: f64,
        previousTime: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanRestoreNotification", (e, info, currentTime, previousTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        duration: f64,
        loopMode: crate::UnityEngine::Playables::DirectorWrapMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, duration, loopMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn PrepareFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareFrame", (playable, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn Restore_internal(
        e: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Restore_internal", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortNotifications(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortNotifications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncDurationWithExternalSource(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncDurationWithExternalSource", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerNotificationsInRange(
        &mut self,
        start: f64,
        end: f64,
        info: crate::UnityEngine::Playables::FrameData,
        playable: crate::UnityEngine::Playables::Playable,
        checkState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TriggerNotificationsInRange",
                (start, end, info, playable, checkState),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Trigger_internal(
        playable: crate::UnityEngine::Playables::Playable,
        output: crate::UnityEngine::Playables::PlayableOutput,
        e: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Trigger_internal", (playable, output, e))?;
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
    pub fn set_timeSource(
        &mut self,
        value: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeSource", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimeNotificationBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeNotificationBehaviour_NotificationEntry {
    pub _cordl_time: f64,
    pub payload: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification>,
    pub notificationFired: bool,
    pub flags: crate::UnityEngine::Timeline::NotificationFlags,
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry =>
    "UnityEngine.Timeline"."TimeNotificationBehaviour/NotificationEntry"
);
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
impl crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    pub fn get_prewarm(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_prewarm",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerInEditor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_triggerInEditor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerOnce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_triggerOnce",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
