#[cfg(feature = "System+Threading+Timer+Scheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct Timer_Scheduler {
    __cordl_parent: crate::System::Object,
    pub needReSort: bool,
    pub list: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Threading::Timer,
    >,
    pub current_next_run: i64,
    pub changed: *mut crate::System::Threading::ManualResetEvent,
}
#[cfg(feature = "System+Threading+Timer+Scheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Timer_Scheduler =>
    "System.Threading"."Timer/Scheduler"
);
#[cfg(feature = "System+Threading+Timer+Scheduler")]
impl std::ops::Deref for crate::System::Threading::Timer_Scheduler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timer+Scheduler")]
impl std::ops::DerefMut for crate::System::Threading::Timer_Scheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timer+Scheduler")]
impl crate::System::Threading::Timer_Scheduler {
    pub fn Add(
        &mut self,
        timer: *mut crate::System::Threading::Timer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (timer))?;
        Ok(__cordl_ret)
    }
    pub fn Change(
        &mut self,
        timer: *mut crate::System::Threading::Timer,
        new_next_run: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Change", (timer, new_next_run))?;
        Ok(__cordl_ret)
    }
    pub fn FireTimer(
        &mut self,
        timer: *mut crate::System::Threading::Timer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireTimer", (timer))?;
        Ok(__cordl_ret)
    }
    pub fn InitScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitScheduler", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalRemove(
        &mut self,
        timer: *mut crate::System::Threading::Timer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRemove", (timer))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        timer: *mut crate::System::Threading::Timer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (timer))?;
        Ok(__cordl_ret)
    }
    pub fn RunSchedulerLoop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RunSchedulerLoop", ())?;
        Ok(__cordl_ret)
    }
    pub fn SchedulerThread(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SchedulerThread", ())?;
        Ok(__cordl_ret)
    }
    pub fn WakeupScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WakeupScheduler", ())?;
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
#[cfg(feature = "System+Threading+Timer+Scheduler")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Timer_Scheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Timer")]
#[repr(C)]
#[derive(Debug)]
pub struct Timer {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub callback: *mut crate::System::Threading::TimerCallback,
    pub state: *mut crate::System::Object,
    pub due_time_ms: i64,
    pub period_ms: i64,
    pub next_run: i64,
    pub disposed: bool,
    pub is_dead: bool,
    pub is_added: bool,
}
#[cfg(feature = "System+Threading+Timer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Timer => "System.Threading"
    ."Timer"
);
#[cfg(feature = "System+Threading+Timer")]
impl std::ops::Deref for crate::System::Threading::Timer {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl std::ops::DerefMut for crate::System::Threading::Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl crate::System::Threading::Timer {
    pub const MaxValue: i64 = 4294967294i64;
    #[cfg(feature = "System+Threading+Timer+Scheduler")]
    pub type Scheduler = crate::System::Threading::Timer_Scheduler;
    #[cfg(feature = "System+Threading+Timer+TimerComparer")]
    pub type TimerComparer = crate::System::Threading::Timer_TimerComparer;
    pub fn Change_TimeSpan_TimeSpan1(
        &mut self,
        dueTime: crate::System::TimeSpan,
        period: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Change", (dueTime, period))?;
        Ok(__cordl_ret)
    }
    pub fn Change_i32_i32_0(
        &mut self,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Change", (dueTime, period))?;
        Ok(__cordl_ret)
    }
    pub fn Change_i64_i64__cordl_bool2(
        &mut self,
        dueTime: i64,
        period: i64,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Change", (dueTime, period, first))?;
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
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        callback: *mut crate::System::Threading::TimerCallback,
        state: *mut crate::System::Object,
        dueTime: i64,
        period: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (callback, state, dueTime, period))?;
        Ok(__cordl_ret)
    }
    pub fn KeepRootedWhileScheduled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeepRootedWhileScheduled", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_TimeSpan_TimeSpan1(
        callback: *mut crate::System::Threading::TimerCallback,
        state: *mut crate::System::Object,
        dueTime: crate::System::TimeSpan,
        period: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_0(
        callback: *mut crate::System::Threading::TimerCallback,
        state: *mut crate::System::Object,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_TimeSpan_TimeSpan1(
        &mut self,
        callback: *mut crate::System::Threading::TimerCallback,
        state: *mut crate::System::Object,
        dueTime: crate::System::TimeSpan,
        period: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_0(
        &mut self,
        callback: *mut crate::System::Threading::TimerCallback,
        state: *mut crate::System::Object,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Timer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Timer_TimerComparer {}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Timer_TimerComparer =>
    "System.Threading"."Timer/TimerComparer"
);
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Timer_TimerComparer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
impl crate::System::Threading::Timer_TimerComparer {
    pub fn Compare(
        &mut self,
        tx: *mut crate::System::Threading::Timer,
        ty: *mut crate::System::Threading::Timer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (tx, ty),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IComparer_Compare(
        &mut self,
        x: *mut crate::System::Object,
        y: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IComparer.Compare",
            (x, y),
        )?;
        Ok(__cordl_ret)
    }
}
