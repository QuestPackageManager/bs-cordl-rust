#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardTaskContinuation {
    __cordl_parent: crate::System::Threading::Tasks::TaskContinuation,
    pub m_task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub m_options: crate::System::Threading::Tasks::TaskContinuationOptions,
    pub m_taskScheduler: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskScheduler,
    >,
}
#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::StandardTaskContinuation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "StandardTaskContinuation";
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
#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
impl std::ops::Deref for crate::System::Threading::Tasks::StandardTaskContinuation {
    type Target = crate::System::Threading::Tasks::TaskContinuation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::StandardTaskContinuation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
impl crate::System::Threading::Tasks::StandardTaskContinuation {
    pub fn New(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        options: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (task, options, scheduler))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        completedTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        bCanInlineContinuationTask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (completedTask, bCanInlineContinuationTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        options: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (task, options, scheduler))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+StandardTaskContinuation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::StandardTaskContinuation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
