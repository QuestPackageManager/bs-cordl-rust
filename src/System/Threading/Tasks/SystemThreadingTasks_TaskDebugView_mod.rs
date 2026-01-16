#[cfg(feature = "cordl_class_System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemThreadingTasks_TaskDebugView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Threading::Tasks::SystemThreadingTasks_TaskDebugView
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "SystemThreadingTasks_TaskDebugView";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
impl std::ops::Deref for crate::System::Threading::Tasks::SystemThreadingTasks_TaskDebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::SystemThreadingTasks_TaskDebugView {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
impl crate::System::Threading::Tasks::SystemThreadingTasks_TaskDebugView {}
#[cfg(feature = "cordl_class_System+Threading+Tasks+SystemThreadingTasks_TaskDebugView")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Threading::Tasks::SystemThreadingTasks_TaskDebugView
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
