#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ProcessThreadCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >,
}
#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::ProcessThreadCollection =>
    "System.Diagnostics"."ProcessThreadCollection"
);
#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
impl std::ops::Deref for crate::System::Diagnostics::ProcessThreadCollection {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
impl std::ops::DerefMut for crate::System::Diagnostics::ProcessThreadCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
impl crate::System::Diagnostics::ProcessThreadCollection {}
#[cfg(feature = "System+Diagnostics+ProcessThreadCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::ProcessThreadCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
