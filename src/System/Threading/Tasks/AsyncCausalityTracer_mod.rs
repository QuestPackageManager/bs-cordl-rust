#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncCausalityTracer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityTracer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::AsyncCausalityTracer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "AsyncCausalityTracer";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Threading::Tasks::CausalityTraceLevel,
                    i32,
                    crate::System::Threading::Tasks::AsyncCausalityStatus,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TraceOperationCompletion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TraceOperationCompletion", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (traceLevel, taskId, status))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceOperationCreation(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        taskId: i32,
        operationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        relatedContext: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Threading::Tasks::CausalityTraceLevel,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    u64,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("TraceOperationCreation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TraceOperationCreation", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (traceLevel, taskId, operationName, relatedContext),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceSynchronousWorkCompletion(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        work: crate::System::Threading::Tasks::CausalitySynchronousWork,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Threading::Tasks::CausalityTraceLevel,
                    crate::System::Threading::Tasks::CausalitySynchronousWork,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("TraceSynchronousWorkCompletion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TraceSynchronousWorkCompletion", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (traceLevel, work))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceSynchronousWorkStart(
        traceLevel: crate::System::Threading::Tasks::CausalityTraceLevel,
        taskId: i32,
        work: crate::System::Threading::Tasks::CausalitySynchronousWork,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Threading::Tasks::CausalityTraceLevel,
                    i32,
                    crate::System::Threading::Tasks::CausalitySynchronousWork,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TraceSynchronousWorkStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TraceSynchronousWorkStart", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (traceLevel, taskId, work))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_LoggingOn() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_LoggingOn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_LoggingOn", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
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
