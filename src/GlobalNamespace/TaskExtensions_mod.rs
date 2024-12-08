#[cfg(feature = "TaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TaskExtensions => ""."TaskExtensions"
);
#[cfg(feature = "TaskExtensions")]
impl std::ops::Deref for TaskExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TaskExtensions")]
impl std::ops::DerefMut for TaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TaskExtensions")]
impl TaskExtensions {
    #[cfg(feature = "TaskExtensions+_WaitAsyncInternal_d__3")]
    pub type _WaitAsyncInternal_d__3 = crate::GlobalNamespace::TaskExtensions__WaitAsyncInternal_d__3;
    #[cfg(feature = "TaskExtensions+__c__DisplayClass4_0_1")]
    pub type __c__DisplayClass4_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::TaskExtensions___c__DisplayClass4_0_1<
        T,
    >;
    #[cfg(feature = "TaskExtensions+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::TaskExtensions___c__DisplayClass3_0;
    #[cfg(feature = "TaskExtensions+_WaitAsyncInternal_d__4_1")]
    pub type _WaitAsyncInternal_d__4_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::TaskExtensions__WaitAsyncInternal_d__4_1<
        T,
    >;
}
#[cfg(feature = "TaskExtensions")]
impl quest_hook::libil2cpp::ObjectType for TaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
