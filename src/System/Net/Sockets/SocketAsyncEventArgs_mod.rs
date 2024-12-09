#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct SocketAsyncEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub disposed: bool,
    pub in_progress: i32,
    pub remote_ep: *mut crate::System::Net::EndPoint,
    pub current_socket: *mut crate::System::Net::Sockets::Socket,
    pub socket_async_result: *mut crate::System::Net::Sockets::SocketAsyncResult,
    pub _ConnectByNameError_k__BackingField: *mut crate::System::Exception,
    pub _AcceptSocket_k__BackingField: *mut crate::System::Net::Sockets::Socket,
    pub _BytesTransferred_k__BackingField: i32,
    pub _DisconnectReuseSocket_k__BackingField: bool,
    pub _LastOperation_k__BackingField: crate::System::Net::Sockets::SocketAsyncOperation,
    pub _ReceiveMessageFromPacketInfo_k__BackingField: crate::System::Net::Sockets::IPPacketInformation,
    pub _SendPacketsElements_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Net::Sockets::SendPacketsElement,
    >,
    pub _SendPacketsFlags_k__BackingField: crate::System::Net::Sockets::TransmitFileOptions,
    pub _SendPacketsSendSize_k__BackingField: i32,
    pub _SocketError_k__BackingField: crate::System::Net::Sockets::SocketError,
    pub _SocketFlags_k__BackingField: crate::System::Net::Sockets::SocketFlags,
    pub _UserToken_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
    pub Completed: *mut crate::System::EventHandler_1<
        *mut crate::System::Net::Sockets::SocketAsyncEventArgs,
    >,
    pub _buffer: crate::System::Memory_1<u8>,
    pub _offset: i32,
    pub _count: i32,
    pub _bufferIsExplicitArray: bool,
    pub _bufferList: *mut crate::System::Collections::Generic::IList_1<
        crate::System::ArraySegment_1<u8>,
    >,
    pub _bufferListInternal: *mut crate::System::Collections::Generic::List_1<
        crate::System::ArraySegment_1<u8>,
    >,
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketAsyncEventArgs =>
    "System.Net.Sockets"."SocketAsyncEventArgs"
);
#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
impl std::ops::Deref for crate::System::Net::Sockets::SocketAsyncEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
impl std::ops::DerefMut for crate::System::Net::Sockets::SocketAsyncEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
impl crate::System::Net::Sockets::SocketAsyncEventArgs {
    pub fn Complete_internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete_internal", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool0(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (flowExecutionContext))?;
        Ok(__cordl_object)
    }
    pub fn OnCompleted(
        &mut self,
        e: *mut crate::System::Net::Sockets::SocketAsyncEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", (e))?;
        Ok(__cordl_ret)
    }
    pub fn SetBuffer(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBuffer", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn SetBytesTransferred(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBytesTransferred", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurrentSocket(
        &mut self,
        socket: *mut crate::System::Net::Sockets::Socket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurrentSocket", (socket))?;
        Ok(__cordl_ret)
    }
    pub fn SetLastOperation(
        &mut self,
        op: crate::System::Net::Sockets::SocketAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLastOperation", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (flowExecutionContext))?;
        Ok(__cordl_ret)
    }
    pub fn add_Completed(
        &mut self,
        value: *mut crate::System::EventHandler_1<
            *mut crate::System::Net::Sockets::SocketAsyncEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Completed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_AcceptSocket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Sockets::Socket> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Sockets::Socket = __cordl_object
            .invoke("get_AcceptSocket", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BufferList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            crate::System::ArraySegment_1<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::System::ArraySegment_1<u8>,
        > = __cordl_object.invoke("get_BufferList", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BytesTransferred(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BytesTransferred", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentSocket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Sockets::Socket> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Sockets::Socket = __cordl_object
            .invoke("get_CurrentSocket", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemoryBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Memory_1<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Memory_1<u8> = __cordl_object
            .invoke("get_MemoryBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Offset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Offset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SocketError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::SocketError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::SocketError = __cordl_object
            .invoke("get_SocketError", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_UserToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_Completed(
        &mut self,
        value: *mut crate::System::EventHandler_1<
            *mut crate::System::Net::Sockets::SocketAsyncEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Completed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AcceptSocket(
        &mut self,
        value: *mut crate::System::Net::Sockets::Socket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AcceptSocket", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_BytesTransferred(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BytesTransferred", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LastOperation(
        &mut self,
        value: crate::System::Net::Sockets::SocketAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LastOperation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RemoteEndPoint(
        &mut self,
        value: *mut crate::System::Net::EndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RemoteEndPoint", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SendPacketsSendSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendPacketsSendSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SocketError(
        &mut self,
        value: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SocketError", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SocketFlags(
        &mut self,
        value: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SocketFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UserToken(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserToken", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::SocketAsyncEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
