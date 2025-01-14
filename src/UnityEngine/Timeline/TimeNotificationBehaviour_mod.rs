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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimeNotificationBehaviour {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimeNotificationBehaviour";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f64,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Playables::INotification,
                    >,
                    crate::UnityEngine::Timeline::NotificationFlags,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddNotification")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddNotification", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_time, payload, flags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanRestoreNotification(
        e: crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        info: crate::UnityEngine::Playables::FrameData,
        currentTime: f64,
        previousTime: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
                    crate::UnityEngine::Playables::FrameData,
                    f64,
                    f64,
                ),
                bool,
                4usize,
            >("CanRestoreNotification")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CanRestoreNotification", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (e, info, currentTime, previousTime))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Playables::PlayableGraph,
                    f64,
                    crate::UnityEngine::Playables::DirectorWrapMode,
                ),
                crate::UnityEngine::Playables::ScriptPlayable_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Timeline::TimeNotificationBehaviour,
                    >,
                >,
                3usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        > = unsafe { method.invoke_unchecked((), (graph, duration, loopMode)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnBehaviourPause")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBehaviourPause", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnGraphStart(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnGraphStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnGraphStart", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PrepareFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PrepareFrame", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Restore_internal(
        e: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Restore_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Restore_internal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SortNotifications(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SortNotifications")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SortNotifications", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SyncDurationWithExternalSource(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SyncDurationWithExternalSource")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SyncDurationWithExternalSource", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f64,
                    f64,
                    crate::UnityEngine::Playables::FrameData,
                    crate::UnityEngine::Playables::Playable,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("TriggerNotificationsInRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TriggerNotificationsInRange", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (start, end, info, playable, checkState))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Trigger_internal(
        playable: crate::UnityEngine::Playables::Playable,
        output: crate::UnityEngine::Playables::PlayableOutput,
        e: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::PlayableOutput,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Trigger_internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Trigger_internal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (playable, output, e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_timeSource(
        &mut self,
        value: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_timeSource")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_timeSource", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeNotificationBehaviour_NotificationEntry {
    pub _cordl_time: f64,
    pub payload: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::INotification>,
    pub notificationFired: bool,
    pub flags: crate::UnityEngine::Timeline::NotificationFlags,
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimeNotificationBehaviour/NotificationEntry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeNotificationBehaviour+NotificationEntry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::TimeNotificationBehaviour_NotificationEntry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_prewarm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_prewarm", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerInEditor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_triggerInEditor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_triggerInEditor", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerOnce(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_triggerOnce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_triggerOnce", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
