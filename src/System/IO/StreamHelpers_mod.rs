#[cfg(feature = "System+IO+StreamHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct StreamHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+StreamHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::StreamHelpers => "System.IO"
    ."StreamHelpers"
);
#[cfg(feature = "System+IO+StreamHelpers")]
impl std::ops::Deref for crate::System::IO::StreamHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamHelpers")]
impl std::ops::DerefMut for crate::System::IO::StreamHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamHelpers")]
impl crate::System::IO::StreamHelpers {}
#[cfg(feature = "System+IO+StreamHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::StreamHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
