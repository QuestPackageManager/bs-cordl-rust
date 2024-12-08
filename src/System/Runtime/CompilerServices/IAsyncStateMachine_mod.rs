#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsyncStateMachine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::IAsyncStateMachine =>
    "System.Runtime.CompilerServices"."IAsyncStateMachine"
);
#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::IAsyncStateMachine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::IAsyncStateMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
impl crate::System::Runtime::CompilerServices::IAsyncStateMachine {
    pub fn SetStateMachine(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStateMachine", (stateMachine))?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IAsyncStateMachine")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::IAsyncStateMachine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
