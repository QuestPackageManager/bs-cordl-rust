#[cfg(feature = "Zenject+BindingUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+BindingUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindingUtil => "Zenject"."BindingUtil"
);
#[cfg(feature = "Zenject+BindingUtil")]
impl std::ops::Deref for crate::Zenject::BindingUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindingUtil")]
impl std::ops::DerefMut for crate::Zenject::BindingUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindingUtil")]
impl crate::Zenject::BindingUtil {}
#[cfg(feature = "Zenject+BindingUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::BindingUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
