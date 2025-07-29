#[cfg(feature = "cordl_class_System+Threading+Tasks+Task+WhenAllPromise")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_WhenAllPromise {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >,
    pub m_tasks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    >,
    pub m_count: i32,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+Task+WhenAllPromise")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Task_WhenAllPromise {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "Task/WhenAllPromise";
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
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl std::ops::Deref for crate::GlobalNamespace::Task_WhenAllPromise {
    type Target = crate::System::Threading::Tasks::Task_1<
        crate::System::Threading::Tasks::VoidTaskResult,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl std::ops::DerefMut for crate::GlobalNamespace::Task_WhenAllPromise {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl crate::GlobalNamespace::Task_WhenAllPromise {
    pub fn Invoke(
        &mut self,
        ignored: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ignored))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tasks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tasks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_InvokeMayRunArbitraryCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InvokeMayRunArbitraryCode", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldNotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("get_ShouldNotifyDebuggerOfWaitCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ShouldNotifyDebuggerOfWaitCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+Task+WhenAllPromise")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Task_WhenAllPromise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl AsRef<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::GlobalNamespace::Task_WhenAllPromise {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
impl AsMut<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::GlobalNamespace::Task_WhenAllPromise {
    fn as_mut(&mut self) -> &mut crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
