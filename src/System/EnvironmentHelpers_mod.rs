#[cfg(feature = "System+EnvironmentHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+EnvironmentHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::EnvironmentHelpers => "System"
    ."EnvironmentHelpers"
);
#[cfg(feature = "System+EnvironmentHelpers")]
impl std::ops::Deref for crate::System::EnvironmentHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+EnvironmentHelpers")]
impl std::ops::DerefMut for crate::System::EnvironmentHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+EnvironmentHelpers")]
impl crate::System::EnvironmentHelpers {}
#[cfg(feature = "System+EnvironmentHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::EnvironmentHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}