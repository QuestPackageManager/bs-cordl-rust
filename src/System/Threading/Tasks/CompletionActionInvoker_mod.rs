#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
#[repr(C)]
#[derive(Debug)]
pub struct CompletionActionInvoker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_action: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::ITaskCompletionAction,
    >,
    pub m_completingTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::CompletionActionInvoker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "CompletionActionInvoker";
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
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl std::ops::Deref for crate::System::Threading::Tasks::CompletionActionInvoker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl crate::System::Threading::Tasks::CompletionActionInvoker {
    pub fn MarkAborted(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadAbortException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Threading::ThreadAbortException,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("MarkAborted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MarkAborted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, completingTask))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::ITaskCompletionAction,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action, completingTask))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+CompletionActionInvoker")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::CompletionActionInvoker {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
