#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct StateMachineAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _StateMachineType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::StateMachineAttribute =>
    "System.Runtime.CompilerServices"."StateMachineAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::StateMachineAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::StateMachineAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
impl crate::System::Runtime::CompilerServices::StateMachineAttribute {
    pub fn New(
        stateMachineType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stateMachineType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        stateMachineType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stateMachineType))?;
        Ok(__cordl_ret)
    }
    pub fn get_StateMachineType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_StateMachineType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StateMachineAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::StateMachineAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
