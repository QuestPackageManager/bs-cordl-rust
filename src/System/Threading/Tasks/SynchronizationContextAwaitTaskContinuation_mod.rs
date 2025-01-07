#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
#[repr(C)]
#[derive(Debug)]
pub struct SynchronizationContextAwaitTaskContinuation {
    __cordl_parent: crate::System::Threading::Tasks::AwaitTaskContinuation,
    pub m_syncContext: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SynchronizationContext,
    >,
}
#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::SynchronizationContextAwaitTaskContinuation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "SynchronizationContextAwaitTaskContinuation";
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
#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
impl std::ops::Deref
for crate::System::Threading::Tasks::SynchronizationContextAwaitTaskContinuation {
    type Target = crate::System::Threading::Tasks::AwaitTaskContinuation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::SynchronizationContextAwaitTaskContinuation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
impl crate::System::Threading::Tasks::SynchronizationContextAwaitTaskContinuation {
    pub fn GetPostActionCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ContextCallback,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPostActionCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, action, flowExecutionContext))?;
        Ok(__cordl_object.into())
    }
    pub fn PostAction(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PostAction", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(
        &mut self,
        ignored: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        canInlineContinuationTask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (ignored, canInlineContinuationTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, action, flowExecutionContext))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+SynchronizationContextAwaitTaskContinuation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::SynchronizationContextAwaitTaskContinuation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
