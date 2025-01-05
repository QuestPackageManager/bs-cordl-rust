#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncStateMachineAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::StateMachineAttribute,
    >,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncStateMachineAttribute =>
    "System.Runtime.CompilerServices"."AsyncStateMachineAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::StateMachineAttribute,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
impl crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    pub fn New(
        stateMachineType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stateMachineType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        stateMachineType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stateMachineType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
