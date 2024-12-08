#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::UnityWebRequestExtensions
    => "BGLib.UnityExtension"."UnityWebRequestExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    #[cfg(
        feature = "BGLib+UnityExtension+UnityWebRequestExtensions+_SendWebRequestAsync_d__0"
    )]
    pub type _SendWebRequestAsync_d__0 = crate::BGLib::UnityExtension::UnityWebRequestExtensions__SendWebRequestAsync_d__0;
    #[cfg(
        feature = "BGLib+UnityExtension+UnityWebRequestExtensions+__c__DisplayClass1_0"
    )]
    pub type __c__DisplayClass1_0 = crate::BGLib::UnityExtension::UnityWebRequestExtensions___c__DisplayClass1_0;
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
