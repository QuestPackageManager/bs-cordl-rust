#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct SocketAsyncResult {
    __cordl_parent: crate::System::IOAsyncResult,
    pub socket: *mut crate::System::Net::Sockets::Socket,
    pub operation: crate::System::Net::Sockets::SocketOperation,
    pub DelayedException: *mut crate::System::Exception,
    pub EndPoint: *mut crate::System::Net::EndPoint,
    pub Buffer: crate::System::Memory_1<u8>,
    pub Offset: i32,
    pub Size: i32,
    pub SockFlags: crate::System::Net::Sockets::SocketFlags,
    pub AcceptSocket: *mut crate::System::Net::Sockets::Socket,
    pub Addresses: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Net::IPAddress,
    >,
    pub Port: i32,
    pub Buffers: *mut crate::System::Collections::Generic::IList_1<
        crate::System::ArraySegment_1<u8>,
    >,
    pub ReuseSocket: bool,
    pub CurrentAddress: i32,
    pub AcceptedSocket: *mut crate::System::Net::Sockets::Socket,
    pub Total: i32,
    pub error: i32,
    pub EndCalled: i32,
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SocketAsyncResult =>
    "System.Net.Sockets"."SocketAsyncResult"
);
#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
impl std::ops::Deref for crate::System::Net::Sockets::SocketAsyncResult {
    type Target = crate::System::IOAsyncResult;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
impl std::ops::DerefMut for crate::System::Net::Sockets::SocketAsyncResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
impl crate::System::Net::Sockets::SocketAsyncResult {
    pub fn CheckIfThrowDelayedException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfThrowDelayedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteDisposed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteDisposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Exception4(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Exception__cordl_bool3(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        synch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (e, synch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Socket5(
        &mut self,
        s: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Socket_i32_6(
        &mut self,
        s: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        total: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (s, total))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete__cordl_bool1(
        &mut self,
        synch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (synch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Complete_i32_2(
        &mut self,
        total: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (total))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        operation: crate::System::Net::Sockets::SocketOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (socket, callback, state, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Socket_AsyncCallback_Il2CppObject_SocketOperation1(
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        operation: crate::System::Net::Sockets::SocketOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (socket, callback, state, operation))?;
        Ok(__cordl_object.into())
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
    pub fn _ctor_Socket_AsyncCallback_Il2CppObject_SocketOperation1(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        operation: crate::System::Net::Sockets::SocketOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (socket, callback, state, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ErrorCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::SocketError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::SocketError = __cordl_object
            .invoke("get_ErrorCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_Handle", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+SocketAsyncResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::SocketAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
