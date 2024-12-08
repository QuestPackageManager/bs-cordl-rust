#[cfg(feature = "AsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AsyncHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AsyncHelper => ""."AsyncHelper"
);
#[cfg(feature = "AsyncHelper")]
impl std::ops::Deref for AsyncHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncHelper")]
impl std::ops::DerefMut for AsyncHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncHelper")]
impl AsyncHelper {
    #[cfg(feature = "AsyncHelper+_AnyTaskTrueNonAlloc_d__2")]
    pub type _AnyTaskTrueNonAlloc_d__2 = crate::GlobalNamespace::AsyncHelper__AnyTaskTrueNonAlloc_d__2;
}
#[cfg(feature = "AsyncHelper")]
impl quest_hook::libil2cpp::ObjectType for AsyncHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
