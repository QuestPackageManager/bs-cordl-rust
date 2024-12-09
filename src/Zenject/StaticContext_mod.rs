#[cfg(feature = "Zenject+StaticContext")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+StaticContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::StaticContext => "Zenject"
    ."StaticContext"
);
#[cfg(feature = "Zenject+StaticContext")]
impl std::ops::Deref for crate::Zenject::StaticContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticContext")]
impl std::ops::DerefMut for crate::Zenject::StaticContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+StaticContext")]
impl crate::Zenject::StaticContext {}
#[cfg(feature = "Zenject+StaticContext")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::StaticContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
