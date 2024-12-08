#[cfg(feature = "Mono+DependencyInjector")]
#[repr(C)]
#[derive(Debug)]
pub struct DependencyInjector {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+DependencyInjector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::DependencyInjector => "Mono"
    ."DependencyInjector"
);
#[cfg(feature = "Mono+DependencyInjector")]
impl std::ops::Deref for crate::Mono::DependencyInjector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+DependencyInjector")]
impl std::ops::DerefMut for crate::Mono::DependencyInjector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+DependencyInjector")]
impl crate::Mono::DependencyInjector {}
#[cfg(feature = "Mono+DependencyInjector")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::DependencyInjector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
