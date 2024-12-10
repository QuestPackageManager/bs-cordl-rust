#[cfg(feature = "System+Net+WebOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct WebOperation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Request_k__BackingField: *mut crate::System::Net::HttpWebRequest,
    pub _Connection_k__BackingField: *mut crate::System::Net::WebConnection,
    pub _ServicePoint_k__BackingField: *mut crate::System::Net::ServicePoint,
    pub _WriteBuffer_k__BackingField: *mut crate::System::Net::BufferOffsetSize,
    pub _IsNtlmChallenge_k__BackingField: bool,
    pub cts: *mut crate::System::Threading::CancellationTokenSource,
    pub requestTask: *mut crate::System::Net::WebCompletionSource_1<
        *mut crate::System::Net::WebRequestStream,
    >,
    pub requestWrittenTask: *mut crate::System::Net::WebCompletionSource_1<
        *mut crate::System::Net::WebRequestStream,
    >,
    pub responseTask: *mut crate::System::Net::WebCompletionSource_1<
        *mut crate::System::Net::WebResponseStream,
    >,
    pub finishedTask: *mut crate::System::Net::WebCompletionSource_1<
        crate::System::ValueTuple_2<bool, *mut crate::System::Net::WebOperation>,
    >,
    pub writeStream: *mut crate::System::Net::WebRequestStream,
    pub responseStream: *mut crate::System::Net::WebResponseStream,
    pub disposedInfo: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    pub closedInfo: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    pub priorityRequest: *mut crate::System::Net::WebOperation,
    pub requestSent: i32,
    pub finished: i32,
}
#[cfg(feature = "System+Net+WebOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebOperation => "System.Net"
    ."WebOperation"
);
#[cfg(feature = "System+Net+WebOperation")]
impl std::ops::Deref for crate::System::Net::WebOperation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebOperation")]
impl std::ops::DerefMut for crate::System::Net::WebOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebOperation")]
impl crate::System::Net::WebOperation {
    #[cfg(feature = "System+Net+WebOperation+_GetRequestStream_d__50")]
    pub type _GetRequestStream_d__50 = crate::System::Net::WebOperation__GetRequestStream_d__50;
    #[cfg(feature = "System+Net+WebOperation+_Run_d__58")]
    pub type _Run_d__58 = crate::System::Net::WebOperation__Run_d__58;
    pub fn Abort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Abort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDisposed(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("CheckDisposed", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckThrowDisposed(
        &mut self,
        throwIt: bool,
        field: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("CheckThrowDisposed", (throwIt, field))?;
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
    pub fn CompleteRequestWritten(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequestStream>,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteRequestWritten", (stream, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
        ok: bool,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (ok, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        > = __cordl_object.invoke("GetRequestStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestStreamInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::WebRequestStream,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::WebRequestStream,
            >,
        > = __cordl_object.invoke("GetRequestStreamInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResponseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::WebResponseStream,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Net::WebResponseStream,
            >,
        > = __cordl_object.invoke("GetResponseStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        writeBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
        isNtlmChallenge: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (request, writeBuffer, isNtlmChallenge, cancellationToken),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterRequest(
        &mut self,
        servicePoint: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterRequest", (servicePoint, connection))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCanceled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCanceled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDisposed(
        &mut self,
        field: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            bool,
        > = __cordl_object.invoke("SetDisposed", (field))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetError(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetError", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPriorityRequest(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPriorityRequest", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfClosedOrDisposed_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfClosedOrDisposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfClosedOrDisposed_CancellationToken1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfClosedOrDisposed", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfDisposed_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfDisposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfDisposed_CancellationToken1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfDisposed", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _RegisterRequest_b__48_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<RegisterRequest>b__48_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        writeBuffer: quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
        isNtlmChallenge: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (request, writeBuffer, isNtlmChallenge, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Aborted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Aborted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Closed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Closed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Connection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection> = __cordl_object
            .invoke("get_Connection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Finished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::WebCompletionSource_1<
                crate::System::ValueTuple_2<bool, *mut crate::System::Net::WebOperation>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebCompletionSource_1<
                crate::System::ValueTuple_2<bool, *mut crate::System::Net::WebOperation>,
            >,
        > = __cordl_object.invoke("get_Finished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNtlmChallenge(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNtlmChallenge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Request(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest> = __cordl_object
            .invoke("get_Request", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint> = __cordl_object
            .invoke("get_ServicePoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::BufferOffsetSize>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::BufferOffsetSize,
        > = __cordl_object.invoke("get_WriteBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WriteStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebRequestStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebRequestStream,
        > = __cordl_object.invoke("get_WriteStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Connection(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Connection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ServicePoint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ServicePoint", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebOperation")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
