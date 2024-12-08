#[cfg(feature = "Zenject+ValidationUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+ValidationUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ValidationUtil => "Zenject"
    ."ValidationUtil"
);
#[cfg(feature = "Zenject+ValidationUtil")]
impl std::ops::Deref for crate::Zenject::ValidationUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ValidationUtil")]
impl std::ops::DerefMut for crate::Zenject::ValidationUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ValidationUtil")]
impl crate::Zenject::ValidationUtil {
    #[cfg(feature = "Zenject+ValidationUtil+__c")]
    pub type __c = crate::Zenject::ValidationUtil___c;
}
#[cfg(feature = "Zenject+ValidationUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ValidationUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
