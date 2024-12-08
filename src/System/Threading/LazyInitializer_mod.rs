#[cfg(feature = "System+Threading+LazyInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyInitializer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Threading+LazyInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::LazyInitializer =>
    "System.Threading"."LazyInitializer"
);
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::Deref for crate::System::Threading::LazyInitializer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::DerefMut for crate::System::Threading::LazyInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl crate::System::Threading::LazyInitializer {}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::LazyInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
