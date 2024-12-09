#[cfg(feature = "BatchExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BatchExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BatchExtensions => ""
    ."BatchExtensions"
);
#[cfg(feature = "BatchExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BatchExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BatchExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BatchExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BatchExtensions")]
impl crate::GlobalNamespace::BatchExtensions {
    #[cfg(feature = "BatchExtensions+_Batch_d__0_1")]
    pub type _Batch_d__0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BatchExtensions__Batch_d__0_1<
        T,
    >;
}
#[cfg(feature = "BatchExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BatchExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
