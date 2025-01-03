#[cfg(feature = "BackgroundCommandQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundCommandQueue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sync: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _commandsQueue: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::GlobalNamespace::IBackgroundCommand,
    >,
    pub _isRunning: bool,
}
#[cfg(feature = "BackgroundCommandQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BackgroundCommandQueue => ""
    ."BackgroundCommandQueue"
);
#[cfg(feature = "BackgroundCommandQueue")]
impl std::ops::Deref for crate::GlobalNamespace::BackgroundCommandQueue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl std::ops::DerefMut for crate::GlobalNamespace::BackgroundCommandQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl crate::GlobalNamespace::BackgroundCommandQueue {
    pub fn Enqueue(
        &mut self,
        command: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBackgroundCommand>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (command))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RunInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RunInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BackgroundCommandQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
