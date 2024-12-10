#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
#[repr(C)]
#[derive(Debug)]
pub struct ReaderWriterLockSlim {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub fIsReentrant: bool,
    pub myLock: i32,
    pub numWriteWaiters: u32,
    pub numReadWaiters: u32,
    pub numWriteUpgradeWaiters: u32,
    pub numUpgradeWaiters: u32,
    pub fNoWaiters: bool,
    pub upgradeLockOwnerId: i32,
    pub writeLockOwnerId: i32,
    pub writeEvent: *mut crate::System::Threading::EventWaitHandle,
    pub readEvent: *mut crate::System::Threading::EventWaitHandle,
    pub upgradeEvent: *mut crate::System::Threading::EventWaitHandle,
    pub waitUpgradeEvent: *mut crate::System::Threading::EventWaitHandle,
    pub lockID: i64,
    pub fUpgradeThreadHoldingRead: bool,
    pub owners: u32,
    pub fDisposed: bool,
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ReaderWriterLockSlim =>
    "System.Threading"."ReaderWriterLockSlim"
);
#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
impl std::ops::Deref for crate::System::Threading::ReaderWriterLockSlim {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
impl std::ops::DerefMut for crate::System::Threading::ReaderWriterLockSlim {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
impl crate::System::Threading::ReaderWriterLockSlim {
    #[cfg(feature = "System+Threading+ReaderWriterLockSlim+TimeoutTracker")]
    pub type TimeoutTracker = crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker;
    pub fn ClearUpgraderWaiting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearUpgraderWaiting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearWriterAcquired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearWriterAcquired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearWritersWaiting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearWritersWaiting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterMyLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnterMyLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterMyLockSpin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnterMyLockSpin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterReadLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnterReadLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterUpgradeableReadLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnterUpgradeableReadLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterWriteLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnterWriteLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitAndWakeUpAppropriateReadWaiters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitAndWakeUpAppropriateReadWaiters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitAndWakeUpAppropriateWaiters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitAndWakeUpAppropriateWaiters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitAndWakeUpAppropriateWaitersPreferringWriters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitAndWakeUpAppropriateWaitersPreferringWriters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitMyLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitMyLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitReadLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitReadLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitUpgradeableReadLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitUpgradeableReadLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitWriteLock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExitWriteLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumReaders(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetNumReaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetThreadRWCount(
        &mut self,
        dontAllocate: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ReaderWriterCount>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ReaderWriterCount,
        > = __cordl_object.invoke("GetThreadRWCount", (dontAllocate))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeThreadCounts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeThreadCounts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRwHashEntryChanged(
        &mut self,
        lrwc: quest_hook::libil2cpp::Gc<crate::System::Threading::ReaderWriterCount>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRwHashEntryChanged", (lrwc))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWriterAcquired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWriterAcquired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LazyCreateEvent(
        &mut self,
        waitEvent: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::EventWaitHandle,
        >,
        makeAutoResetEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyCreateEvent", (waitEvent, makeAutoResetEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_LockRecursionPolicy1(
        recursionPolicy: crate::System::Threading::LockRecursionPolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recursionPolicy))?;
        Ok(__cordl_object.into())
    }
    pub fn SetUpgraderWaiting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUpgraderWaiting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWriterAcquired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWriterAcquired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWritersWaiting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWritersWaiting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterReadLockCore(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterReadLockCore", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterReadLock_ReaderWriterLockSlim_TimeoutTracker1(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryEnterReadLock", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterReadLock_i32_0(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterReadLock", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterUpgradeableReadLockCore(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterUpgradeableReadLockCore", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterUpgradeableReadLock_ReaderWriterLockSlim_TimeoutTracker1(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterUpgradeableReadLock", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterUpgradeableReadLock_i32_0(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterUpgradeableReadLock", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterWriteLockCore(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterWriteLockCore", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterWriteLock_ReaderWriterLockSlim_TimeoutTracker1(
        &mut self,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryEnterWriteLock", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnterWriteLock_i32_0(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryEnterWriteLock", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnEvent(
        &mut self,
        waitEvent: quest_hook::libil2cpp::Gc<crate::System::Threading::EventWaitHandle>,
        numWaiters: quest_hook::libil2cpp::ByRefMut<u32>,
        timeout: crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker,
        isWriteWaiter: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("WaitOnEvent", (waitEvent, numWaiters, timeout, isWriteWaiter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_LockRecursionPolicy1(
        &mut self,
        recursionPolicy: crate::System::Threading::LockRecursionPolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recursionPolicy))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadLockHeld(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadLockHeld", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUpgradeableReadLockHeld(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsUpgradeableReadLockHeld", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsWriteLockHeld(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsWriteLockHeld", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecursiveReadCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecursiveReadCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecursiveUpgradeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecursiveUpgradeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecursiveWriteCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecursiveWriteCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WaitingReadCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WaitingReadCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WaitingUpgradeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WaitingUpgradeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WaitingWriteCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WaitingWriteCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ReaderWriterLockSlim {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim+TimeoutTracker")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReaderWriterLockSlim_TimeoutTracker {
    pub m_total: i32,
    pub m_start: i32,
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim+TimeoutTracker")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ReaderWriterLockSlim_TimeoutTracker => "System.Threading"
    ."ReaderWriterLockSlim/TimeoutTracker"
);
#[cfg(feature = "System+Threading+ReaderWriterLockSlim+TimeoutTracker")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+ReaderWriterLockSlim+TimeoutTracker")]
impl crate::System::Threading::ReaderWriterLockSlim_TimeoutTracker {
    pub fn _ctor(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (millisecondsTimeout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsExpired",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemainingMilliseconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RemainingMilliseconds",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
