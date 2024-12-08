#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
#[repr(C)]
#[derive(Debug)]
pub struct IRuntimeVariables {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::IRuntimeVariables =>
    "System.Runtime.CompilerServices"."IRuntimeVariables"
);
#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::IRuntimeVariables {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::IRuntimeVariables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
impl crate::System::Runtime::CompilerServices::IRuntimeVariables {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IRuntimeVariables")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::IRuntimeVariables {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
