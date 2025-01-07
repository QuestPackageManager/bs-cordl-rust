#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct DebuggerSupport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::DebuggerSupport {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "DebuggerSupport";
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
#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
impl std::ops::Deref for crate::System::Threading::Tasks::DebuggerSupport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::DebuggerSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
impl crate::System::Threading::Tasks::DebuggerSupport {
    pub fn AddToActiveTasks(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddToActiveTasks", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToActiveTasksNonInlined(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddToActiveTasksNonInlined", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromActiveTasks(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveFromActiveTasks", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromActiveTasksNonInlined(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveFromActiveTasksNonInlined", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceOperationCompletion(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        status: crate::Internal::Runtime::Augments::AsyncStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceOperationCompletion", (traceLevel, task, status))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceOperationCreation(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        relatedContext: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TraceOperationCreation",
                (traceLevel, task, operationName, relatedContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceOperationRelation(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        relation: crate::System::Threading::Tasks::CausalityRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceOperationRelation", (traceLevel, task, relation))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceSynchronousWorkCompletion(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        work: crate::System::Threading::Tasks::CausalitySynchronousWork,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceSynchronousWorkCompletion", (traceLevel, work))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceSynchronousWorkStart(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        work: crate::System::Threading::Tasks::CausalitySynchronousWork,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceSynchronousWorkStart", (traceLevel, task, work))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LoggingOn() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LoggingOn", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+DebuggerSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::DebuggerSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
