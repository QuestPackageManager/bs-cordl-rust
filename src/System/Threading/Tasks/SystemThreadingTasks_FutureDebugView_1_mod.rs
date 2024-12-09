#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemThreadingTasks_FutureDebugView_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::SystemThreadingTasks_FutureDebugView_1 < TResult > =>
    "System.Threading.Tasks"."SystemThreadingTasks_FutureDebugView`1" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::SystemThreadingTasks_FutureDebugView_1<TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::SystemThreadingTasks_FutureDebugView_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::SystemThreadingTasks_FutureDebugView_1<TResult> {}
#[cfg(feature = "System+Threading+Tasks+SystemThreadingTasks_FutureDebugView_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::SystemThreadingTasks_FutureDebugView_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
