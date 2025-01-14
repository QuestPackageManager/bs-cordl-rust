#[cfg(feature = "System+Threading+Timer")]
#[repr(C)]
#[derive(Debug)]
pub struct Timer {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
    pub state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub due_time_ms: i64,
    pub period_ms: i64,
    pub next_run: i64,
    pub disposed: bool,
    pub is_dead: bool,
    pub is_added: bool,
}
#[cfg(feature = "System+Threading+Timer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::Timer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "Timer";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan, crate::System::TimeSpan),
                bool,
                2usize,
            >("Change")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Change", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (dueTime, period))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change_i32_i32_0(
        &mut self,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("Change")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Change", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (dueTime, period))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change_i64_i64__cordl_bool2(
        &mut self,
        dueTime: i64,
        period: i64,
        first: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64, i64, bool), bool, 3usize>("Change")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Change", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (dueTime, period, first))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Threading::Tasks::ValueTask,
                0usize,
            >("DisposeAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisposeAsync", 0usize
                )
            });
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeMonotonic() -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i64, 0usize>("GetTimeMonotonic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTimeMonotonic", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dueTime: i64,
        period: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i64,
                    i64,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback, state, dueTime, period))
        };
        Ok(__cordl_ret.into())
    }
    pub fn KeepRootedWhileScheduled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("KeepRootedWhileScheduled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "KeepRootedWhileScheduled", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_TimeSpan_TimeSpan1(
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dueTime: crate::System::TimeSpan,
        period: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_0(
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, state, dueTime, period))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_TimeSpan_TimeSpan1(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dueTime: crate::System::TimeSpan,
        period: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::TimeSpan,
                    crate::System::TimeSpan,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback, state, dueTime, period))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dueTime: i32,
        period: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::TimerCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback, state, dueTime, period))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_scheduler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer_Scheduler>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Timer_Scheduler>,
                0usize,
            >("get_scheduler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_scheduler", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Timer_Scheduler,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Threading+Timer")]
impl AsRef<crate::System::IAsyncDisposable> for crate::System::Threading::Timer {
    fn as_ref(&self) -> &crate::System::IAsyncDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl AsMut<crate::System::IAsyncDisposable> for crate::System::Threading::Timer {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl AsRef<crate::System::IDisposable> for crate::System::Threading::Timer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Timer")]
impl AsMut<crate::System::IDisposable> for crate::System::Threading::Timer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Timer+Scheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct Timer_Scheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub needReSort: bool,
    pub list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
        >,
    >,
    pub current_next_run: i64,
    pub changed: quest_hook::libil2cpp::Gc<crate::System::Threading::ManualResetEvent>,
}
#[cfg(feature = "System+Threading+Timer+Scheduler")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::Timer_Scheduler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "Timer/Scheduler";
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
#[cfg(feature = "System+Threading+Timer+Scheduler")]
impl std::ops::Deref for crate::System::Threading::Timer_Scheduler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change(
        &mut self,
        timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
        new_next_run: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>, i64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Change")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Change", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timer, new_next_run))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FireTimer(
        &mut self,
        timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FireTimer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FireTimer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InitScheduler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitScheduler", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRemove(
        &mut self,
        timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalRemove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalRemove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Remove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Remove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RunSchedulerLoop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("RunSchedulerLoop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RunSchedulerLoop", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn SchedulerThread(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SchedulerThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SchedulerThread", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn TimerCB(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("TimerCB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TimerCB", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (o))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WakeupScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("WakeupScheduler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WakeupScheduler", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer_Scheduler>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Timer_Scheduler>,
                0usize,
            >("get_Instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Timer_Scheduler,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Timer_TimerComparer {}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Timer_TimerComparer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "Timer/TimerComparer";
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
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::Timer_TimerComparer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::Timer_TimerComparer {
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
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::Timer_TimerComparer {
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
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::Timer_TimerComparer {
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
        tx: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
        ty: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
                    quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
                ),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (tx, ty)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IComparer_Compare(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i32,
                2usize,
            >("System.Collections.IComparer.Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Collections.IComparer.Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    >,
> for crate::System::Threading::Timer_TimerComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    >,
> for crate::System::Threading::Timer_TimerComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
impl AsRef<crate::System::Collections::IComparer>
for crate::System::Threading::Timer_TimerComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Timer+TimerComparer")]
impl AsMut<crate::System::Collections::IComparer>
for crate::System::Threading::Timer_TimerComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        todo!()
    }
}
