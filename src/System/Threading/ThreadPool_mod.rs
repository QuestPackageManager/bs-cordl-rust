#[cfg(feature = "System+Threading+ThreadPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+ThreadPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadPool =>
    "System.Threading"."ThreadPool"
);
#[cfg(feature = "System+Threading+ThreadPool")]
impl std::ops::Deref for crate::System::Threading::ThreadPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl crate::System::Threading::ThreadPool {
    #[cfg(feature = "System+Threading+ThreadPool+__c__DisplayClass17_0_1")]
    pub type __c__DisplayClass17_0_1<TState: quest_hook::libil2cpp::Type> = crate::System::Threading::ThreadPool___c__DisplayClass17_0_1<
        TState,
    >;
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ThreadPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
