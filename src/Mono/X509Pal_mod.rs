#[cfg(feature = "Mono+X509Pal")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Pal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+X509Pal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::X509Pal => "Mono"."X509Pal"
);
#[cfg(feature = "Mono+X509Pal")]
impl std::ops::Deref for crate::Mono::X509Pal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl std::ops::DerefMut for crate::Mono::X509Pal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl crate::Mono::X509Pal {}
#[cfg(feature = "Mono+X509Pal")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::X509Pal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
