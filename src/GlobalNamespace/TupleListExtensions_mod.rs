#[cfg(feature = "TupleListExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TupleListExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TupleListExtensions => ""
    ."TupleListExtensions"
);
#[cfg(feature = "TupleListExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TupleListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TupleListExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TupleListExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TupleListExtensions")]
impl crate::GlobalNamespace::TupleListExtensions {}
#[cfg(feature = "TupleListExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TupleListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
