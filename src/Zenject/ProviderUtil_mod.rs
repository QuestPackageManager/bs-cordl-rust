#[cfg(feature = "Zenject+ProviderUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+ProviderUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ProviderUtil => "Zenject"
    ."ProviderUtil"
);
#[cfg(feature = "Zenject+ProviderUtil")]
impl std::ops::Deref for crate::Zenject::ProviderUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderUtil")]
impl std::ops::DerefMut for crate::Zenject::ProviderUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderUtil")]
impl crate::Zenject::ProviderUtil {}
#[cfg(feature = "Zenject+ProviderUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProviderUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
