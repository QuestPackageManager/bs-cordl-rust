#[cfg(feature = "PerformanceReport")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceReport {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PerformanceReport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PerformanceReport => ""."PerformanceReport"
);
#[cfg(feature = "PerformanceReport")]
impl std::ops::Deref for PerformanceReport {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceReport")]
impl std::ops::DerefMut for PerformanceReport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceReport")]
impl PerformanceReport {
    pub const kDefaultFpsWindow: f32 = 1f32;
}
#[cfg(feature = "PerformanceReport")]
impl quest_hook::libil2cpp::ObjectType for PerformanceReport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
