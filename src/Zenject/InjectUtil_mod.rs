#[cfg(feature = "Zenject+InjectUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+InjectUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectUtil => "Zenject"."InjectUtil"
);
#[cfg(feature = "Zenject+InjectUtil")]
impl std::ops::Deref for crate::Zenject::InjectUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectUtil")]
impl std::ops::DerefMut for crate::Zenject::InjectUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectUtil")]
impl crate::Zenject::InjectUtil {
    #[cfg(feature = "Zenject+InjectUtil+__c")]
    pub type __c = crate::Zenject::InjectUtil___c;
}
#[cfg(feature = "Zenject+InjectUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InjectUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
