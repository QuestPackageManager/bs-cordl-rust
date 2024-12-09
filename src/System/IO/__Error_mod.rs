#[cfg(feature = "System+IO+__Error")]
#[repr(C)]
#[derive(Debug)]
pub struct __Error {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+__Error")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::__Error => "System.IO"."__Error"
);
#[cfg(feature = "System+IO+__Error")]
impl std::ops::Deref for crate::System::IO::__Error {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+__Error")]
impl std::ops::DerefMut for crate::System::IO::__Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+__Error")]
impl crate::System::IO::__Error {}
#[cfg(feature = "System+IO+__Error")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::__Error {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
