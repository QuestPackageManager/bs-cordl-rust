#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
#[repr(C)]
#[derive(Debug)]
pub struct Hint {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::CompilerServices::Hint =>
    "Unity.Burst.CompilerServices"."Hint"
);
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::Hint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::Hint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl crate::Unity::Burst::CompilerServices::Hint {}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::CompilerServices::Hint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}