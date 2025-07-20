#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolTaskScheduler {
    __cordl_parent: crate::System::Threading::Tasks::TaskScheduler,
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "ThreadPoolTaskScheduler";
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
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl std::ops::Deref for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    type Target = crate::System::Threading::Tasks::TaskScheduler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyWorkItemProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("NotifyWorkItemProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(), "NotifyWorkItemProgress",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn QueueTask(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("QueueTask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(), "QueueTask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (task))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryDequeue(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>),
                bool,
                1usize,
            >("TryDequeue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(), "TryDequeue", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (task))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryExecuteTaskInline(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        taskWasPreviouslyQueued: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>, bool),
                bool,
                2usize,
            >("TryExecuteTaskInline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(), "TryExecuteTaskInline",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (task, taskWasPreviouslyQueued))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresAtomicStartTransition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::Tasks::ThreadPoolTaskScheduler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_RequiresAtomicStartTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::Tasks::ThreadPoolTaskScheduler as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_RequiresAtomicStartTransition", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
