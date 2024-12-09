#[cfg(feature = "AsyncComputeManager")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncComputeManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _asyncComputeRequests: *mut crate::System::Collections::Concurrent::BlockingCollection_1<
        *mut crate::GlobalNamespace::AsyncComputeOperation,
    >,
    pub _computeThread: *mut crate::System::Threading::Thread,
    pub _disposed: bool,
}
#[cfg(feature = "AsyncComputeManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AsyncComputeManager => ""
    ."AsyncComputeManager"
);
#[cfg(feature = "AsyncComputeManager")]
impl std::ops::Deref for crate::GlobalNamespace::AsyncComputeManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncComputeManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::AsyncComputeManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncComputeManager")]
impl crate::GlobalNamespace::AsyncComputeManager {
    pub fn BeginOperation_AsyncComputeOperation0(
        &mut self,
        operation: *mut crate::GlobalNamespace::AsyncComputeOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginOperation", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn BeginOperation_AsyncComputeOperation_1_1<T>(
        &mut self,
        operation: *mut crate::GlobalNamespace::AsyncComputeOperation_1<T>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<T> = __cordl_object
            .invoke("BeginOperation", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeThreadRun(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeThreadRun", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "AsyncComputeManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AsyncComputeManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
