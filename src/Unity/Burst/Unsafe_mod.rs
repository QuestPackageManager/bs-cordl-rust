#[cfg(feature = "Unity+Burst+Unsafe")]
#[repr(C)]
#[derive(Debug)]
pub struct Unsafe {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Unsafe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Unsafe => "Unity.Burst"."Unsafe"
);
#[cfg(feature = "Unity+Burst+Unsafe")]
impl std::ops::Deref for crate::Unity::Burst::Unsafe {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl std::ops::DerefMut for crate::Unity::Burst::Unsafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl crate::Unity::Burst::Unsafe {}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Unsafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}