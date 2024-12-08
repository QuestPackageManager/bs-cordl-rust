#[cfg(feature = "System+Threading+ReaderWriterLock")]
#[repr(C)]
#[derive(Debug)]
pub struct ReaderWriterLock {
    __cordl_parent: crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject,
    pub seq_num: i32,
    pub state: i32,
    pub readers: i32,
    pub writer_lock_owner: i32,
    pub writer_queue: *mut crate::System::Threading::LockQueue,
    pub reader_locks: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Threading+ReaderWriterLock")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ReaderWriterLock =>
    "System.Threading"."ReaderWriterLock"
);
#[cfg(feature = "System+Threading+ReaderWriterLock")]
impl std::ops::Deref for crate::System::Threading::ReaderWriterLock {
    type Target = crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLock")]
impl std::ops::DerefMut for crate::System::Threading::ReaderWriterLock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLock")]
impl crate::System::Threading::ReaderWriterLock {
    pub fn ReleaseWriterLock_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseWriterLock", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseWriterLock_i32_1(
        &mut self,
        releaseCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseWriterLock", (releaseCount))?;
        Ok(__cordl_ret)
    }
    pub fn AcquireReaderLock_i32_0(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcquireReaderLock", (millisecondsTimeout))?;
        Ok(__cordl_ret)
    }
    pub fn AcquireReaderLock_i32_1(
        &mut self,
        millisecondsTimeout: i32,
        initialLockCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcquireReaderLock", (millisecondsTimeout, initialLockCount))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsWriterLockHeld(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsWriterLockHeld", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasWriterLock(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasWriterLock", ())?;
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
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn AcquireWriterLock_i32_0(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcquireWriterLock", (millisecondsTimeout))?;
        Ok(__cordl_ret)
    }
    pub fn AcquireWriterLock_i32_1(
        &mut self,
        millisecondsTimeout: i32,
        initialLockCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcquireWriterLock", (millisecondsTimeout, initialLockCount))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseReaderLock_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseReaderLock", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseReaderLock_i32_i32_1(
        &mut self,
        currentCount: i32,
        releaseCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseReaderLock", (currentCount, releaseCount))?;
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
#[cfg(feature = "System+Threading+ReaderWriterLock")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ReaderWriterLock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
