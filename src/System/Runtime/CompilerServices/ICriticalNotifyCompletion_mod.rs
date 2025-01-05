#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
#[repr(C)]
#[derive(Debug)]
pub struct ICriticalNotifyCompletion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ICriticalNotifyCompletion =>
    "System.Runtime.CompilerServices"."ICriticalNotifyCompletion"
);
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    pub fn UnsafeOnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsafeOnCompleted", (continuation))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ICriticalNotifyCompletion")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
