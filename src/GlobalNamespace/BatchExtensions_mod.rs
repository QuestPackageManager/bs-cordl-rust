#[cfg(feature = "BatchExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BatchExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BatchExtensions => ""."BatchExtensions"
);
#[cfg(feature = "BatchExtensions")]
impl std::ops::Deref for BatchExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BatchExtensions")]
impl std::ops::DerefMut for BatchExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BatchExtensions")]
impl BatchExtensions {
    #[cfg(feature = "BatchExtensions+_Batch_d__0_1")]
    pub type _Batch_d__0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BatchExtensions__Batch_d__0_1<
        T,
    >;
}
#[cfg(feature = "BatchExtensions")]
impl quest_hook::libil2cpp::ObjectType for BatchExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}