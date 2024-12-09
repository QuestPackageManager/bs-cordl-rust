#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorUtil => "Zenject"
    ."SubContainerCreatorUtil"
);
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl crate::Zenject::SubContainerCreatorUtil {}
#[cfg(feature = "Zenject+SubContainerCreatorUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SubContainerCreatorUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
