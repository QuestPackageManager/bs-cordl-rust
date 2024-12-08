#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTimeProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::DefaultTimeProvider => "BGNet.Core"
    ."DefaultTimeProvider"
);
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl std::ops::Deref for crate::BGNet::Core::DefaultTimeProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl std::ops::DerefMut for crate::BGNet::Core::DefaultTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl crate::BGNet::Core::DefaultTimeProvider {}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Core::DefaultTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
