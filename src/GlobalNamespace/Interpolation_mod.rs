#[cfg(feature = "Interpolation")]
#[repr(C)]
#[derive(Debug)]
pub struct Interpolation {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Interpolation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for Interpolation => ""."Interpolation"
);
#[cfg(feature = "Interpolation")]
impl std::ops::Deref for Interpolation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Interpolation")]
impl std::ops::DerefMut for Interpolation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Interpolation")]
impl Interpolation {}
#[cfg(feature = "Interpolation")]
impl quest_hook::libil2cpp::ObjectType for Interpolation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
