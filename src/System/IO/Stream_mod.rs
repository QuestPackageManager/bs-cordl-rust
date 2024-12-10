#[cfg(feature = "System+IO+Stream")]
#[repr(C)]
#[derive(Debug)]
pub struct Stream {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _activeReadWriteTask: *mut crate::System::IO::Stream_ReadWriteTask,
    pub _asyncActiveSemaphore: *mut crate::System::Threading::SemaphoreSlim,
}
#[cfg(feature = "System+IO+Stream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Stream => "System.IO"."Stream"
);
#[cfg(feature = "System+IO+Stream")]
impl std::ops::Deref for crate::System::IO::Stream {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream")]
impl std::ops::DerefMut for crate::System::IO::Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream")]
impl crate::System::IO::Stream {
    pub const DefaultCopyBufferSize: i32 = 81920i32;
    #[cfg(feature = "System+IO+Stream+NullStream")]
    pub type NullStream = crate::GlobalNamespace::Stream_NullStream;
    #[cfg(feature = "System+IO+Stream+ReadWriteParameters")]
    pub type ReadWriteParameters = crate::System::IO::Stream_ReadWriteParameters;
    #[cfg(feature = "System+IO+Stream+ReadWriteTask")]
    pub type ReadWriteTask = crate::System::IO::Stream_ReadWriteTask;
    #[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
    pub type SynchronousAsyncResult = crate::System::IO::Stream_SynchronousAsyncResult;
    #[cfg(feature = "System+IO+Stream+_CopyToAsyncInternal_d__28")]
    pub type _CopyToAsyncInternal_d__28 = crate::System::IO::Stream__CopyToAsyncInternal_d__28;
    #[cfg(feature = "System+IO+Stream+_FinishWriteAsync_d__57")]
    pub type _FinishWriteAsync_d__57 = crate::System::IO::Stream__FinishWriteAsync_d__57;
    #[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
    pub type __ReadAsync_g__FinishReadAsync_44_0_d = crate::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d;
    #[cfg(feature = "System+IO+Stream+__c")]
    pub type __c = crate::System::IO::Stream___c;
    pub fn BeginEndReadAsync(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object.invoke("BeginEndReadAsync", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginEndWriteAsync(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("BeginEndWriteAsync", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginRead(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginRead", (buffer, offset, count, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginReadInternal(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializeAsynchronously: bool,
        apm: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginReadInternal",
                (buffer, offset, count, callback, state, serializeAsynchronously, apm),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginWrite(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginWrite", (buffer, offset, count, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginWriteInternal(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializeAsynchronously: bool,
        apm: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginWriteInternal",
                (buffer, offset, count, callback, state, serializeAsynchronously, apm),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BlockingBeginRead(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BlockingBeginRead", (buffer, offset, count, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BlockingBeginWrite(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BlockingBeginWrite", (buffer, offset, count, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyToAsyncInternal(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "CopyToAsyncInternal",
                (destination, bufferSize, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyToAsync_Stream0(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("CopyToAsync", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyToAsync_i32_1(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("CopyToAsync", (destination, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyToAsync_i32_CancellationToken2(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("CopyToAsync", (destination, bufferSize, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_Stream0(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_i32_1(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (destination, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsync", ())?;
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
    pub fn EndRead(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndRead", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndWrite(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndWrite", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureAsyncActiveSemaphoreInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SemaphoreSlim>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SemaphoreSlim,
        > = __cordl_object.invoke("EnsureAsyncActiveSemaphoreInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishTrackingAsyncOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishTrackingAsyncOperation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishWriteAsync(
        &mut self,
        writeTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        localBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FinishWriteAsync", (writeTask, localBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FlushAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsync_CancellationToken1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FlushAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCopyBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCopyBufferSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasOverriddenBeginEndRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasOverriddenBeginEndRead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasOverriddenBeginEndWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasOverriddenBeginEndWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadAsync_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object.invoke("ReadAsync", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsync_Il2CppArray_i32_i32_CancellationToken1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object
            .invoke("ReadAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsync_Memory_1_CancellationToken2(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ValueTask_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask_1<i32> = __cordl_object
            .invoke("ReadAsync", (buffer, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_Span_1_1(
        &mut self,
        buffer: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunReadWriteTask(
        &mut self,
        readWriteTask: quest_hook::libil2cpp::Gc<crate::System::IO::Stream_ReadWriteTask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunReadWriteTask", (readWriteTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunReadWriteTaskWhenReady(
        &mut self,
        asyncWaiter: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        readWriteTask: quest_hook::libil2cpp::Gc<crate::System::IO::Stream_ReadWriteTask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunReadWriteTaskWhenReady", (asyncWaiter, readWriteTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek(
        &mut self,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("Seek", (offset, origin))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteAsync", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppArray_i32_i32_CancellationToken1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("WriteAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_ReadOnlyMemory_1_CancellationToken2(
        &mut self,
        buffer: crate::System::ReadOnlyMemory_1<u8>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("WriteAsync", (buffer, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteByte(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_ReadOnlySpan_1_1(
        &mut self,
        buffer: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer))?;
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
    pub fn get_CanRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanRead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanSeek(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanSeek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanTimeout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WriteTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReadTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadTimeout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WriteTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WriteTimeout", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Stream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::Stream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Stream")]
impl AsRef<crate::System::IAsyncDisposable> for crate::System::IO::Stream {
    fn as_ref(&self) -> &crate::System::IAsyncDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream")]
impl AsMut<crate::System::IAsyncDisposable> for crate::System::IO::Stream {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream")]
impl AsRef<crate::System::IDisposable> for crate::System::IO::Stream {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream")]
impl AsMut<crate::System::IDisposable> for crate::System::IO::Stream {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Stream_ReadWriteParameters {
    pub Buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Offset: i32,
    pub Count: i32,
}
#[cfg(feature = "System+IO+Stream+ReadWriteParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Stream_ReadWriteParameters =>
    "System.IO"."Stream/ReadWriteParameters"
);
#[cfg(feature = "System+IO+Stream+ReadWriteParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::IO::Stream_ReadWriteParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteParameters")]
impl crate::System::IO::Stream_ReadWriteParameters {}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
#[repr(C)]
#[derive(Debug)]
pub struct Stream_ReadWriteTask {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<i32>,
    pub _isRead: bool,
    pub _apm: bool,
    pub _stream: *mut crate::System::IO::Stream,
    pub _buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _offset: i32,
    pub _count: i32,
    pub _callback: *mut crate::System::AsyncCallback,
    pub _context: *mut crate::System::Threading::ExecutionContext,
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Stream_ReadWriteTask => "System.IO"
    ."Stream/ReadWriteTask"
);
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl std::ops::Deref for crate::System::IO::Stream_ReadWriteTask {
    type Target = crate::System::Threading::Tasks::Task_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl std::ops::DerefMut for crate::System::IO::Stream_ReadWriteTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl crate::System::IO::Stream_ReadWriteTask {
    pub fn ClearBeginState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBeginState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        isRead: bool,
        apm: bool,
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut quest_hook::libil2cpp::Il2CppObject, i32>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (isRead, apm, function, state, stream, buffer, offset, count, callback),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn System_Threading_Tasks_ITaskCompletionAction_Invoke(
        &mut self,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Threading.Tasks.ITaskCompletionAction.Invoke",
                (completingTask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Threading_Tasks_ITaskCompletionAction_get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Threading.Tasks.ITaskCompletionAction.get_InvokeMayRunArbitraryCode",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        isRead: bool,
        apm: bool,
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut quest_hook::libil2cpp::Il2CppObject, i32>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (isRead, apm, function, state, stream, buffer, offset, count, callback),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::Stream_ReadWriteTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl AsRef<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::IO::Stream_ReadWriteTask {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream+ReadWriteTask")]
impl AsMut<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::IO::Stream_ReadWriteTask {
    fn as_mut(&mut self) -> &mut crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct Stream_SynchronousAsyncResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _stateObject: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _isWrite: bool,
    pub _waitHandle: *mut crate::System::Threading::ManualResetEvent,
    pub _exceptionInfo: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    pub _endXxxCalled: bool,
    pub _bytesRead: i32,
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Stream_SynchronousAsyncResult =>
    "System.IO"."Stream/SynchronousAsyncResult"
);
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl std::ops::Deref for crate::System::IO::Stream_SynchronousAsyncResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl std::ops::DerefMut for crate::System::IO::Stream_SynchronousAsyncResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl crate::System::IO::Stream_SynchronousAsyncResult {
    #[cfg(feature = "System+IO+Stream+SynchronousAsyncResult+__c")]
    pub type __c = crate::System::IO::SynchronousAsyncResult_Stream___c;
    pub fn New_Exception_Il2CppObject__cordl_bool2(
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isWrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ex, asyncStateObject, isWrite))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject1(
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asyncStateObject))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Il2CppObject0(
        bytesRead: i32,
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bytesRead, asyncStateObject))?;
        Ok(__cordl_object.into())
    }
    pub fn ThrowIfError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Exception_Il2CppObject__cordl_bool2(
        &mut self,
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isWrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ex, asyncStateObject, isWrite))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asyncStateObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Il2CppObject0(
        &mut self,
        bytesRead: i32,
        asyncStateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bytesRead, asyncStateObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AsyncState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitHandle,
        > = __cordl_object.invoke("get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedSynchronously(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CompletedSynchronously", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Stream_SynchronousAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl AsRef<crate::System::IAsyncResult>
for crate::System::IO::Stream_SynchronousAsyncResult {
    fn as_ref(&self) -> &crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream+SynchronousAsyncResult")]
impl AsMut<crate::System::IAsyncResult>
for crate::System::IO::Stream_SynchronousAsyncResult {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Stream___ReadAsync_g__FinishReadAsync_44_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder_1<
        i32,
    >,
    pub readTask: *mut crate::System::Threading::Tasks::Task_1<i32>,
    pub localBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub localDestination: crate::System::Memory_1<u8>,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_1_ConfiguredTaskAwaiter<
        i32,
    >,
}
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d => "System.IO"
    ."Stream/<<ReadAsync>g__FinishReadAsync|44_0>d"
);
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
impl crate::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "System+IO+Stream+__ReadAsync_g__FinishReadAsync_44_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::System::IO::Stream___ReadAsync_g__FinishReadAsync_44_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
