#[cfg(feature = "System+IO+PathInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PathInternal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IO+PathInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::PathInternal => "System.IO"
    ."PathInternal"
);
#[cfg(feature = "System+IO+PathInternal")]
impl std::ops::Deref for crate::System::IO::PathInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PathInternal")]
impl std::ops::DerefMut for crate::System::IO::PathInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PathInternal")]
impl crate::System::IO::PathInternal {}
#[cfg(feature = "System+IO+PathInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::PathInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
