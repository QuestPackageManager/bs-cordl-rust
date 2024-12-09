#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
#[repr(C)]
#[derive(Debug)]
pub struct Aliasing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::CompilerServices::Aliasing =>
    "Unity.Burst.CompilerServices"."Aliasing"
);
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::Aliasing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::Aliasing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl crate::Unity::Burst::CompilerServices::Aliasing {}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::Aliasing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
