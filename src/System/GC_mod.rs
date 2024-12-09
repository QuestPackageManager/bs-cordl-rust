#[cfg(feature = "System+GC")]
#[repr(C)]
#[derive(Debug)]
pub struct GC {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+GC")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::GC => "System"."GC"
);
#[cfg(feature = "System+GC")]
impl std::ops::Deref for crate::System::GC {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+GC")]
impl std::ops::DerefMut for crate::System::GC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+GC")]
impl crate::System::GC {}
#[cfg(feature = "System+GC")]
impl quest_hook::libil2cpp::ObjectType for crate::System::GC {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
