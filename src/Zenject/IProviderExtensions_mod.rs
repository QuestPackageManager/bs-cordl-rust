#[cfg(feature = "Zenject+IProviderExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IProviderExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IProviderExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IProviderExtensions => "Zenject"
    ."IProviderExtensions"
);
#[cfg(feature = "Zenject+IProviderExtensions")]
impl std::ops::Deref for crate::Zenject::IProviderExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl std::ops::DerefMut for crate::Zenject::IProviderExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl crate::Zenject::IProviderExtensions {}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IProviderExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
