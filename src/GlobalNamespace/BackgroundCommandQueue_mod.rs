#[cfg(feature = "BackgroundCommandQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundCommandQueue {
    __cordl_parent: crate::System::Object,
    pub _sync: *mut crate::System::Object,
    pub _commandsQueue: *mut crate::System::Collections::Generic::Queue_1<
        *mut IBackgroundCommand,
    >,
    pub _isRunning: bool,
}
#[cfg(feature = "BackgroundCommandQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BackgroundCommandQueue => ""."BackgroundCommandQueue"
);
#[cfg(feature = "BackgroundCommandQueue")]
impl std::ops::Deref for BackgroundCommandQueue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl std::ops::DerefMut for BackgroundCommandQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl BackgroundCommandQueue {
    #[cfg(feature = "BackgroundCommandQueue+_RunInternal_d__4")]
    pub type _RunInternal_d__4 = crate::GlobalNamespace::BackgroundCommandQueue__RunInternal_d__4;
    pub fn RunInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RunInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Enqueue(
        &mut self,
        command: *mut IBackgroundCommand,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (command))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BackgroundCommandQueue")]
impl quest_hook::libil2cpp::ObjectType for BackgroundCommandQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
