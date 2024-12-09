#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectMoqExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectMoqExtensions => "Zenject"
    ."ZenjectMoqExtensions"
);
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl std::ops::Deref for crate::Zenject::ZenjectMoqExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl std::ops::DerefMut for crate::Zenject::ZenjectMoqExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl crate::Zenject::ZenjectMoqExtensions {}
#[cfg(feature = "Zenject+ZenjectMoqExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectMoqExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
