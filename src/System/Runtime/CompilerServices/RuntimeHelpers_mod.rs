#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::RuntimeHelpers =>
    "System.Runtime.CompilerServices"."RuntimeHelpers"
);
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl crate::System::Runtime::CompilerServices::RuntimeHelpers {}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
