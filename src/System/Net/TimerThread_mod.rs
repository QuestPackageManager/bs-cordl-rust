#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerNode_TimerThread_TimerState {
    Cancelled = 2i32,
    Fired = 1i32,
    Ready = 0i32,
    Sentinel = 3i32,
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerNode_TimerThread_TimerState =>
    "System.Net"."TimerThread/TimerNode/TimerState"
);
#[cfg(feature = "System+Net+TimerThread")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+TimerThread")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread => "System.Net"
    ."TimerThread"
);
#[cfg(feature = "System+Net+TimerThread")]
impl std::ops::Deref for crate::System::Net::TimerThread {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread")]
impl std::ops::DerefMut for crate::System::Net::TimerThread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread")]
impl crate::System::Net::TimerThread {
    #[cfg(feature = "System+Net+TimerThread+Callback")]
    pub type Callback = crate::System::Net::TimerThread_Callback;
    #[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
    pub type InfiniteTimer = crate::System::Net::TimerThread_InfiniteTimer;
    #[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
    pub type InfiniteTimerQueue = crate::System::Net::TimerThread_InfiniteTimerQueue;
    #[cfg(feature = "System+Net+TimerThread+Queue")]
    pub type Queue = crate::System::Net::TimerThread_Queue;
    #[cfg(feature = "System+Net+TimerThread+Timer")]
    pub type Timer = crate::System::Net::TimerThread_Timer;
    #[cfg(feature = "System+Net+TimerThread+TimerNode")]
    pub type TimerNode = crate::System::Net::TimerThread_TimerNode;
    #[cfg(feature = "System+Net+TimerThread+TimerQueue")]
    pub type TimerQueue = crate::System::Net::TimerThread_TimerQueue;
}
#[cfg(feature = "System+Net+TimerThread")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_Callback =>
    "System.Net"."TimerThread/Callback"
);
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl std::ops::Deref for crate::System::Net::TimerThread_Callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl crate::System::Net::TimerThread_Callback {
    pub fn Invoke(
        &mut self,
        timer: *mut crate::System::Net::TimerThread_Timer,
        timeNoticed: i32,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (timer, timeNoticed, context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_InfiniteTimer {
    __cordl_parent: crate::System::Net::TimerThread_Timer,
    pub cancelled: i32,
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_InfiniteTimer =>
    "System.Net"."TimerThread/InfiniteTimer"
);
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl std::ops::Deref for crate::System::Net::TimerThread_InfiniteTimer {
    type Target = crate::System::Net::TimerThread_Timer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_InfiniteTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl crate::System::Net::TimerThread_InfiniteTimer {
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
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
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::TimerThread_InfiniteTimer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_InfiniteTimerQueue {
    __cordl_parent: crate::System::Net::TimerThread_Queue,
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_InfiniteTimerQueue =>
    "System.Net"."TimerThread/InfiniteTimerQueue"
);
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl std::ops::Deref for crate::System::Net::TimerThread_InfiniteTimerQueue {
    type Target = crate::System::Net::TimerThread_Queue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_InfiniteTimerQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl crate::System::Net::TimerThread_InfiniteTimerQueue {
    pub fn CreateTimer(
        &mut self,
        callback: *mut crate::System::Net::TimerThread_Callback,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_Timer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_Timer = __cordl_object
            .invoke("CreateTimer", (callback, context))?;
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
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::TimerThread_InfiniteTimerQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Queue {
    __cordl_parent: crate::System::Object,
    pub m_DurationMilliseconds: i32,
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_Queue => "System.Net"
    ."TimerThread/Queue"
);
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl std::ops::Deref for crate::System::Net::TimerThread_Queue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Queue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl crate::System::Net::TimerThread_Queue {
    pub fn CreateTimer(
        &mut self,
        callback: *mut crate::System::Net::TimerThread_Callback,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_Timer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_Timer = __cordl_object
            .invoke("CreateTimer", (callback, context))?;
        Ok(__cordl_ret)
    }
    pub fn New(durationMilliseconds: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret)
    }
    pub fn get_Duration(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Duration", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Queue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Timer {
    __cordl_parent: crate::System::Object,
    pub m_StartTimeMilliseconds: i32,
    pub m_DurationMilliseconds: i32,
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_Timer => "System.Net"
    ."TimerThread/Timer"
);
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl std::ops::Deref for crate::System::Net::TimerThread_Timer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl crate::System::Net::TimerThread_Timer {
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(durationMilliseconds: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret)
    }
    pub fn get_Expiration(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Expiration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StartTime(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_StartTime", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Timer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_TimerNode {
    __cordl_parent: crate::System::Net::TimerThread_Timer,
    pub m_TimerState: crate::System::Net::TimerNode_TimerThread_TimerState,
    pub m_Callback: *mut crate::System::Net::TimerThread_Callback,
    pub m_Context: *mut crate::System::Object,
    pub m_QueueLock: *mut crate::System::Object,
    pub next: *mut crate::System::Net::TimerThread_TimerNode,
    pub prev: *mut crate::System::Net::TimerThread_TimerNode,
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_TimerNode =>
    "System.Net"."TimerThread/TimerNode"
);
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl std::ops::Deref for crate::System::Net::TimerThread_TimerNode {
    type Target = crate::System::Net::TimerThread_Timer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_TimerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl crate::System::Net::TimerThread_TimerNode {
    #[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
    pub type TimerState = crate::System::Net::TimerNode_TimerThread_TimerState;
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fire(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fire", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_TimerThread_Callback_Object_i32_Object0(
        callback: *mut crate::System::Net::TimerThread_Callback,
        context: *mut crate::System::Object,
        durationMilliseconds: i32,
        queueLock: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, context, durationMilliseconds, queueLock))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TimerThread_Callback_Object_i32_Object0(
        &mut self,
        callback: *mut crate::System::Net::TimerThread_Callback,
        context: *mut crate::System::Object,
        durationMilliseconds: i32,
        queueLock: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, context, durationMilliseconds, queueLock))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_TimerNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_TimerNode = __cordl_object
            .invoke("get_Next", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_TimerNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_TimerNode = __cordl_object
            .invoke("get_Prev", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Next(
        &mut self,
        value: *mut crate::System::Net::TimerThread_TimerNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Next", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Prev(
        &mut self,
        value: *mut crate::System::Net::TimerThread_TimerNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Prev", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_TimerNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_TimerQueue {
    __cordl_parent: crate::System::Net::TimerThread_Queue,
    pub m_ThisHandle: crate::System::IntPtr,
    pub m_Timers: *mut crate::System::Net::TimerThread_TimerNode,
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::TimerThread_TimerQueue =>
    "System.Net"."TimerThread/TimerQueue"
);
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl std::ops::Deref for crate::System::Net::TimerThread_TimerQueue {
    type Target = crate::System::Net::TimerThread_Queue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_TimerQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl crate::System::Net::TimerThread_TimerQueue {
    pub fn CreateTimer(
        &mut self,
        callback: *mut crate::System::Net::TimerThread_Callback,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::TimerThread_Timer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::TimerThread_Timer = __cordl_object
            .invoke("CreateTimer", (callback, context))?;
        Ok(__cordl_ret)
    }
    pub fn Fire(
        &mut self,
        nextExpiration: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fire", (nextExpiration))?;
        Ok(__cordl_ret)
    }
    pub fn New(durationMilliseconds: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_TimerQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
