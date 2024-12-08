#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenUtilInternal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ZenUtilInternal =>
    "Zenject.Internal"."ZenUtilInternal"
);
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::Deref for crate::Zenject::Internal::ZenUtilInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::DerefMut for crate::Zenject::Internal::ZenUtilInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl crate::Zenject::Internal::ZenUtilInternal {
    #[cfg(feature = "Zenject+Internal+ZenUtilInternal+_GetAllSceneContexts_d__3")]
    pub type _GetAllSceneContexts_d__3 = crate::Zenject::Internal::ZenUtilInternal__GetAllSceneContexts_d__3;
    #[cfg(feature = "Zenject+Internal+ZenUtilInternal+__c")]
    pub type __c = crate::Zenject::Internal::ZenUtilInternal___c;
    #[cfg(feature = "Zenject+Internal+ZenUtilInternal+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::Zenject::Internal::ZenUtilInternal___c__DisplayClass10_0;
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ZenUtilInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
