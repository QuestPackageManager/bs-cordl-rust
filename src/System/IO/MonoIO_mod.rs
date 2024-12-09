#[cfg(feature = "System+IO+MonoIO")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+MonoIO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MonoIO => "System.IO"."MonoIO"
);
#[cfg(feature = "System+IO+MonoIO")]
impl std::ops::Deref for crate::System::IO::MonoIO {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl std::ops::DerefMut for crate::System::IO::MonoIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl crate::System::IO::MonoIO {}
#[cfg(feature = "System+IO+MonoIO")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::MonoIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
