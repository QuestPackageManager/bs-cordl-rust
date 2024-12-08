#[cfg(feature = "System+Runtime+RuntimeImports")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeImports {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::RuntimeImports =>
    "System.Runtime"."RuntimeImports"
);
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl std::ops::Deref for crate::System::Runtime::RuntimeImports {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl std::ops::DerefMut for crate::System::Runtime::RuntimeImports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl crate::System::Runtime::RuntimeImports {}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::RuntimeImports {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
