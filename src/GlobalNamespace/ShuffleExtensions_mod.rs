#[cfg(feature = "ShuffleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ShuffleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ShuffleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShuffleExtensions => ""
    ."ShuffleExtensions"
);
#[cfg(feature = "ShuffleExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ShuffleExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShuffleExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShuffleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShuffleExtensions")]
impl crate::GlobalNamespace::ShuffleExtensions {
    #[cfg(feature = "ShuffleExtensions+_PickRandomElementsWithTombstone_d__1_1")]
    pub type _PickRandomElementsWithTombstone_d__1_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::ShuffleExtensions__PickRandomElementsWithTombstone_d__1_1<
        T,
    >;
    #[cfg(feature = "ShuffleExtensions+_TakeWithTombstone_d__2_1")]
    pub type _TakeWithTombstone_d__2_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::ShuffleExtensions__TakeWithTombstone_d__2_1<
        T,
    >;
    #[cfg(feature = "ShuffleExtensions+_ZipSkipTombstone_d__3")]
    pub type _ZipSkipTombstone_d__3 = crate::GlobalNamespace::ShuffleExtensions__ZipSkipTombstone_d__3;
}
#[cfg(feature = "ShuffleExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ShuffleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
