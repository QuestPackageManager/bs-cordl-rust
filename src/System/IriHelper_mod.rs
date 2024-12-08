#[cfg(feature = "System+IriHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IriHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IriHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IriHelper => "System"."IriHelper"
);
#[cfg(feature = "System+IriHelper")]
impl std::ops::Deref for crate::System::IriHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IriHelper")]
impl std::ops::DerefMut for crate::System::IriHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IriHelper")]
impl crate::System::IriHelper {}
#[cfg(feature = "System+IriHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IriHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
