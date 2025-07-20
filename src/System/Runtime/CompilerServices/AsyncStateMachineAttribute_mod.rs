#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncStateMachineAttribute {
    __cordl_parent: crate::System::Runtime::CompilerServices::StateMachineAttribute,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "AsyncStateMachineAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncStateMachineAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute {
    type Target = crate::System::Runtime::CompilerServices::StateMachineAttribute;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::CompilerServices::AsyncStateMachineAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::CompilerServices::AsyncStateMachineAttribute
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (stateMachineType))?
        };
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
