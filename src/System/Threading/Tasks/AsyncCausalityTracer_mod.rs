#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncCausalityTracer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::AsyncCausalityTracer
    => "System.Threading.Tasks"."AsyncCausalityTracer"
);
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
impl std::ops::Deref for crate::System::Threading::Tasks::AsyncCausalityTracer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::AsyncCausalityTracer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
impl crate::System::Threading::Tasks::AsyncCausalityTracer {
    pub fn TraceOperationCompletion(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        taskId: i32,
        status: crate::System::Threading::Tasks::AsyncCausalityStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceOperationCompletion", (traceLevel, taskId, status))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceOperationCreation(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        taskId: i32,
        operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        relatedContext: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TraceOperationCreation",
                (traceLevel, taskId, operationName, relatedContext),
            )?;
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
        taskId: i32,
        work: crate::System::Threading::Tasks::CausalitySynchronousWork,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceSynchronousWorkStart", (traceLevel, taskId, work))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LoggingOn() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LoggingOn", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::AsyncCausalityTracer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
