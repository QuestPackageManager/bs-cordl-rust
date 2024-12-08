#[cfg(feature = "System+IO+MonoLinqHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoLinqHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IO+MonoLinqHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MonoLinqHelper => "System.IO"
    ."MonoLinqHelper"
);
#[cfg(feature = "System+IO+MonoLinqHelper")]
impl std::ops::Deref for crate::System::IO::MonoLinqHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoLinqHelper")]
impl std::ops::DerefMut for crate::System::IO::MonoLinqHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoLinqHelper")]
impl crate::System::IO::MonoLinqHelper {}
#[cfg(feature = "System+IO+MonoLinqHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::MonoLinqHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
