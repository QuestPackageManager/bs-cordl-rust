#[cfg(feature = "System+Activator")]
#[repr(C)]
#[derive(Debug)]
pub struct Activator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Activator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Activator => "System"."Activator"
);
#[cfg(feature = "System+Activator")]
impl std::ops::Deref for crate::System::Activator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Activator")]
impl std::ops::DerefMut for crate::System::Activator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Activator")]
impl crate::System::Activator {}
#[cfg(feature = "System+Activator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Activator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
