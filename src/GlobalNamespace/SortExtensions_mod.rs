#[cfg(feature = "SortExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SortExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SortExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SortExtensions => ""."SortExtensions"
);
#[cfg(feature = "SortExtensions")]
impl std::ops::Deref for SortExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl std::ops::DerefMut for SortExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl SortExtensions {
    #[cfg(feature = "SortExtensions+__c__DisplayClass1_0_1")]
    pub type __c__DisplayClass1_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::SortExtensions___c__DisplayClass1_0_1<
        T,
    >;
}
#[cfg(feature = "SortExtensions")]
impl quest_hook::libil2cpp::ObjectType for SortExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
