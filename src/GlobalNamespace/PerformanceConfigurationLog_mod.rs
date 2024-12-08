#[cfg(feature = "PerformanceConfigurationLog")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationLog {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PerformanceConfigurationLog")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PerformanceConfigurationLog => ""
    ."PerformanceConfigurationLog"
);
#[cfg(feature = "PerformanceConfigurationLog")]
impl std::ops::Deref for PerformanceConfigurationLog {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationLog")]
impl std::ops::DerefMut for PerformanceConfigurationLog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationLog")]
impl PerformanceConfigurationLog {}
#[cfg(feature = "PerformanceConfigurationLog")]
impl quest_hook::libil2cpp::ObjectType for PerformanceConfigurationLog {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
