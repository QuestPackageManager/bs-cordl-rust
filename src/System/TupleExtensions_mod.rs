#[cfg(feature = "System+TupleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+TupleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TupleExtensions => "System"
    ."TupleExtensions"
);
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::Deref for crate::System::TupleExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::DerefMut for crate::System::TupleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl crate::System::TupleExtensions {}
#[cfg(feature = "System+TupleExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TupleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
