#[cfg(feature = "CoroutineHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CoroutineHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "CoroutineHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CoroutineHelpers => ""."CoroutineHelpers"
);
#[cfg(feature = "CoroutineHelpers")]
impl std::ops::Deref for CoroutineHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineHelpers")]
impl std::ops::DerefMut for CoroutineHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineHelpers")]
impl CoroutineHelpers {
    #[cfg(feature = "CoroutineHelpers+_ExecuteAfterDelayCoroutine_d__0")]
    pub type _ExecuteAfterDelayCoroutine_d__0 = crate::GlobalNamespace::CoroutineHelpers__ExecuteAfterDelayCoroutine_d__0;
}
#[cfg(feature = "CoroutineHelpers")]
impl quest_hook::libil2cpp::ObjectType for CoroutineHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}