#[cfg(feature = "AssertExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AssertExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AssertExtensions => ""
    ."AssertExtensions"
);
#[cfg(feature = "AssertExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::AssertExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AssertExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::AssertExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AssertExtensions")]
impl crate::GlobalNamespace::AssertExtensions {
    pub const kUnityAssertions: &'static str = "UNITY_ASSERTIONS";
}
#[cfg(feature = "AssertExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AssertExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
