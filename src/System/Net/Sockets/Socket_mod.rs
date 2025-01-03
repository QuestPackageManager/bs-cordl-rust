#[cfg(feature = "System+Net+Sockets+Socket")]
#[repr(C)]
#[derive(Debug)]
pub struct Socket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedTaskEventArgs: *mut crate::System::Net::Sockets::Socket_CachedEventArgs,
    pub is_closed: bool,
    pub is_listening: bool,
    pub useOverlappedIO: bool,
    pub linger_timeout: i32,
    pub addressFamily: crate::System::Net::Sockets::AddressFamily,
    pub socketType: crate::System::Net::Sockets::SocketType,
    pub protocolType: crate::System::Net::Sockets::ProtocolType,
    pub m_Handle: *mut crate::System::Net::Sockets::SafeSocketHandle,
    pub seed_endpoint: *mut crate::System::Net::EndPoint,
    pub ReadSem: *mut crate::System::Threading::SemaphoreSlim,
    pub WriteSem: *mut crate::System::Threading::SemaphoreSlim,
    pub is_blocking: bool,
    pub is_bound: bool,
    pub is_connected: bool,
    pub m_IntCleanedUp: i32,
    pub connect_in_progress: bool,
    pub _cordl_ID: i32,
}
#[cfg(feature = "System+Net+Sockets+Socket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::Socket =>
    "System.Net.Sockets"."Socket"
);
#[cfg(feature = "System+Net+Sockets+Socket")]
impl std::ops::Deref for crate::System::Net::Sockets::Socket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket")]
impl std::ops::DerefMut for crate::System::Net::Sockets::Socket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket")]
impl crate::System::Net::Sockets::Socket {
    pub const DefaultCloseTimeout: i32 = -1i32;
    pub const SOCKET_CLOSED_CODE: i32 = 10004i32;
    pub const TIMEOUT_EXCEPTION_MSG: &'static str = "A connection attempt failed because the connected party did not properly respondafter a period of time, or established connection failed because connected host has failed to respond";
    #[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
    pub type AwaitableSocketAsyncEventArgs = crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs;
    #[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
    pub type CachedEventArgs = crate::System::Net::Sockets::Socket_CachedEventArgs;
    #[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
    pub type Int32TaskSocketAsyncEventArgs = crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs;
    #[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
    pub type TaskSocketAsyncEventArgs_1<TResult: quest_hook::libil2cpp::Type> = crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<
        TResult,
    >;
    #[cfg(feature = "System+Net+Sockets+Socket+WSABUF")]
    pub type WSABUF = crate::System::Net::Sockets::Socket_WSABUF;
    pub fn AcceptAsync(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AcceptAsync", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn Accept_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket,
        > = __cordl_object.invoke("Accept", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Accept_Socket1(
        &mut self,
        acceptSocket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Accept", (acceptSocket))?;
        Ok(__cordl_ret.into())
    }
    pub fn Accept_icall(
        sock: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Accept_icall", (sock, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Accept_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SafeSocketHandle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Accept_internal", (safeHandle, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Available_icall(
        socket: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Available_icall", (socket, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Available_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Available_internal", (safeHandle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginAccept(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginAccept", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginConnect_EndPoint_AsyncCallback_Il2CppObject1(
        &mut self,
        remoteEP: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginConnect", (remoteEP, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginConnect_Il2CppString_i32_AsyncCallback_Il2CppObject0(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginConnect", (host, port, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginMConnect(
        sockares: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginMConnect", (sockares))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginReceive_AsyncCallback_Il2CppObject0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginReceive",
                (buffer, offset, _cordl_size, socketFlags, callback, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginReceive_ByRefMut_AsyncCallback_Il2CppObject1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginReceive",
                (buffer, offset, _cordl_size, socketFlags, errorCode, callback, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginSConnect(
        sockares: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginSConnect", (sockares))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginSendCallback(
        sockares: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncResult,
        >,
        sent_so_far: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginSendCallback", (sockares, sent_so_far))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginSend_AsyncCallback_Il2CppObject0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginSend",
                (buffer, offset, _cordl_size, socketFlags, callback, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginSend_ByRefMut_AsyncCallback_Il2CppObject1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginSend",
                (buffer, offset, _cordl_size, socketFlags, errorCode, callback, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Bind(
        &mut self,
        localEP: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bind", (localEP))?;
        Ok(__cordl_ret.into())
    }
    pub fn Bind_icall(
        sock: crate::System::IntPtr,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Bind_icall", (sock, sa, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Bind_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Bind_internal", (safeHandle, sa, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blocking_icall(
        socket: crate::System::IntPtr,
        block: bool,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blocking_icall", (socket, block, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blocking_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        block: bool,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blocking_internal", (safeHandle, block, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanTryAddressFamily(
        &mut self,
        family: crate::System::Net::Sockets::AddressFamily,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanTryAddressFamily", (family))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Close_i32_1(
        &mut self,
        timeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close_icall(
        socket: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Close_icall", (socket, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteAccept(
        s: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        saea: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<
                *mut crate::System::Net::Sockets::Socket,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompleteAccept", (s, saea))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteSendReceive(
        s: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        saea: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs,
        >,
        isReceive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompleteSendReceive", (s, saea, isReceive))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_EndPoint1(
        &mut self,
        remoteEP: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Connect", (remoteEP))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_IPAddress_i32_0(
        &mut self,
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Connect", (address, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_icall(
        sock: crate::System::IntPtr,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect_icall", (sock, sa, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect_internal", (safeHandle, sa, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect(
        &mut self,
        reuseSocket: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (reuseSocket))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_icall(
        sock: crate::System::IntPtr,
        reuse: bool,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Disconnect_icall", (sock, reuse, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        reuse: bool,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Disconnect_internal", (safeHandle, reuse, error))?;
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
    pub fn EndAccept_ByRefMut_ByRefMut_IAsyncResult1(
        &mut self,
        buffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        bytesTransferred: quest_hook::libil2cpp::ByRefMut<i32>,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket,
        > = __cordl_object.invoke("EndAccept", (buffer, bytesTransferred, asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndAccept_IAsyncResult0(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket,
        > = __cordl_object.invoke("EndAccept", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndConnect(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndConnect", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndDisconnect(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndDisconnect", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndReceiveFrom_internal(
        &mut self,
        sockares: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncResult,
        >,
        ares: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("EndReceiveFrom_internal", (sockares, ares))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndReceive_ByRefMut1(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("EndReceive", (asyncResult, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndReceive_IAsyncResult0(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndReceive", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndSendTo(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndSendTo", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndSend_ByRefMut1(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("EndSend", (asyncResult, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndSend_IAsyncResult0(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndSend", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetException(
        error: crate::System::Net::Sockets::SocketError,
        wrapExceptionsInIOExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetException", (error, wrapExceptionsInIOExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSocketOption(
        &mut self,
        optionLevel: crate::System::Net::Sockets::SocketOptionLevel,
        optionName: crate::System::Net::Sockets::SocketOptionName,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetSocketOption", (optionLevel, optionName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSocketOption_obj_icall(
        socket: crate::System::IntPtr,
        level: crate::System::Net::Sockets::SocketOptionLevel,
        name: crate::System::Net::Sockets::SocketOptionName,
        obj_val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSocketOption_obj_icall", (socket, level, name, obj_val, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSocketOption_obj_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        level: crate::System::Net::Sockets::SocketOptionLevel,
        name: crate::System::Net::Sockets::SocketOptionName,
        obj_val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSocketOption_obj_internal",
                (safeHandle, level, name, obj_val, error),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IOControl_IOControlCode0(
        &mut self,
        ioControlCode: crate::System::Net::Sockets::IOControlCode,
        optionInValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        optionOutValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IOControl", (ioControlCode, optionInValue, optionOutValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn IOControl_i32_1(
        &mut self,
        ioControlCode: i32,
        optionInValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        optionOutValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IOControl", (ioControlCode, optionInValue, optionOutValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn IOControl_icall(
        sock: crate::System::IntPtr,
        ioctl_code: i32,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IOControl_icall", (sock, ioctl_code, input, output, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn IOControl_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        ioctl_code: i32,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IOControl_internal",
                (safeHandle, ioctl_code, input, output, error),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitSocketAsyncEventArgs(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        operation: crate::System::Net::Sockets::SocketOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitSocketAsyncEventArgs", (e, callback, state, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSockets() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeSockets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalShutdown(
        &mut self,
        how: crate::System::Net::Sockets::SocketShutdown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalShutdown", (how))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsProtocolSupported(
        networkInterface: crate::System::Net::NetworkInformation::NetworkInterfaceComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsProtocolSupported", (networkInterface))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsProtocolSupported_internal(
        networkInterface: crate::System::Net::NetworkInformation::NetworkInterfaceComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsProtocolSupported_internal", (networkInterface))?;
        Ok(__cordl_ret.into())
    }
    pub fn Linger(
        &mut self,
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Linger", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Listen(
        &mut self,
        backlog: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Listen", (backlog))?;
        Ok(__cordl_ret.into())
    }
    pub fn Listen_icall(
        sock: crate::System::IntPtr,
        backlog: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Listen_icall", (sock, backlog, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Listen_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        backlog: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Listen_internal", (safeHandle, backlog, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalEndPoint_icall(
        socket: crate::System::IntPtr,
        family: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalEndPoint_icall", (socket, family, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalEndPoint_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        family: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalEndPoint_internal", (safeHandle, family, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AddressFamily_SocketType_ProtocolType0(
        addressFamily: crate::System::Net::Sockets::AddressFamily,
        socketType: crate::System::Net::Sockets::SocketType,
        protocolType: crate::System::Net::Sockets::ProtocolType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (addressFamily, socketType, protocolType))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SafeSocketHandle1(
        family: crate::System::Net::Sockets::AddressFamily,
        _cordl_type: crate::System::Net::Sockets::SocketType,
        proto: crate::System::Net::Sockets::ProtocolType,
        safe_handle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (family, _cordl_type, proto, safe_handle))?;
        Ok(__cordl_object.into())
    }
    pub fn Poll(
        &mut self,
        microSeconds: i32,
        mode: crate::System::Net::Sockets::SelectMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Poll", (microSeconds, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Poll_icall(
        socket: crate::System::IntPtr,
        mode: crate::System::Net::Sockets::SelectMode,
        timeout: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Poll_icall", (socket, mode, timeout, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Poll_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        mode: crate::System::Net::Sockets::SelectMode,
        timeout: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Poll_internal", (safeHandle, mode, timeout, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueIOSelectorJob(
        &mut self,
        sem: quest_hook::libil2cpp::Gc<crate::System::Threading::SemaphoreSlim>,
        handle: crate::System::IntPtr,
        job: quest_hook::libil2cpp::Gc<crate::System::IOSelectorJob>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueIOSelectorJob", (sem, handle, job))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveAsyncApm(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object.invoke("ReceiveAsyncApm", (buffer, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveAsync_Memory_1_SocketFlags__cordl_bool_CancellationToken0(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        fromNetworkStream: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ValueTask_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask_1<i32> = __cordl_object
            .invoke(
                "ReceiveAsync",
                (buffer, socketFlags, fromNetworkStream, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveAsync_SocketAsyncEventArgs1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReceiveAsync", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveFrom_Il2CppArray0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        remoteEP: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::EndPoint>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ReceiveFrom",
                (buffer, offset, _cordl_size, socketFlags, remoteEP),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveFrom_Il2CppArray_ByRefMut1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        remoteEP: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::EndPoint>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ReceiveFrom",
                (buffer, offset, _cordl_size, socketFlags, remoteEP, errorCode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveFrom_Memory_1_ByRefMut2(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        remoteEP: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::EndPoint>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ReceiveFrom",
                (buffer, offset, _cordl_size, socketFlags, remoteEP, errorCode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveFrom_icall(
        sock: crate::System::IntPtr,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        sockaddr: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Net::SocketAddress,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReceiveFrom_icall",
                (sock, buffer, count, flags, sockaddr, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveFrom_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        sockaddr: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Net::SocketAddress,
        >,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReceiveFrom_internal",
                (safeHandle, buffer, count, flags, sockaddr, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_IList_1_SocketFlags1(
        &mut self,
        buffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::ArraySegment_1<u8>,
            >,
        >,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Receive", (buffers, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_IList_1_SocketFlags_ByRefMut4(
        &mut self,
        buffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::ArraySegment_1<u8>,
            >,
        >,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buffers, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_Il2CppArray_i32_i32_SocketFlags0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buffer, offset, _cordl_size, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_Il2CppArray_i32_i32_SocketFlags_ByRefMut2(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buffer, offset, _cordl_size, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_Memory_1_i32_i32_SocketFlags_ByRefMut3(
        &mut self,
        buffer: crate::System::Memory_1<u8>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buffer, offset, _cordl_size, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_Span_1_SocketFlags_ByRefMut5(
        &mut self,
        buffer: crate::System::Span_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buffer, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_array_icall(
        sock: crate::System::IntPtr,
        bufarray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Receive_array_icall",
                (sock, bufarray, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_icall(
        sock: crate::System::IntPtr,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Receive_icall", (sock, buffer, count, flags, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_internal_SafeSocketHandle_Il2CppObject_i32_SocketFlags_ByRefMut__cordl_bool0(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        bufarray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Receive_internal",
                (safeHandle, bufarray, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive_internal_SafeSocketHandle_Il2CppObject_i32_SocketFlags_ByRefMut__cordl_bool1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Receive_internal",
                (safeHandle, buffer, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemapIPEndPoint(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = __cordl_object
            .invoke("RemapIPEndPoint", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoteEndPoint_icall(
        socket: crate::System::IntPtr,
        family: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoteEndPoint_icall", (socket, family, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoteEndPoint_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        family: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoteEndPoint_internal", (safeHandle, family, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnSocketAsyncEventArgs_Socket_Int32TaskSocketAsyncEventArgs__cordl_bool0(
        &mut self,
        saea: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs,
        >,
        isReceive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnSocketAsyncEventArgs", (saea, isReceive))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnSocketAsyncEventArgs_Socket_TaskSocketAsyncEventArgs_1_1(
        &mut self,
        saea: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<
                *mut crate::System::Net::Sockets::Socket,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnSocketAsyncEventArgs", (saea))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendAsync(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SendAsync", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncApm(
        &mut self,
        buffer: crate::System::ReadOnlyMemory_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object.invoke("SendAsyncApm", (buffer, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncForNetworkStream(
        &mut self,
        buffer: crate::System::ReadOnlyMemory_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke(
                "SendAsyncForNetworkStream",
                (buffer, socketFlags, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTo(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        remoteEP: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendTo", (buffer, offset, _cordl_size, socketFlags, remoteEP))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTo_icall(
        sock: crate::System::IntPtr,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendTo_icall", (sock, buffer, count, flags, sa, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTo_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        sa: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SendTo_internal",
                (safeHandle, buffer, count, flags, sa, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_IList_1_SocketFlags0(
        &mut self,
        buffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::ArraySegment_1<u8>,
            >,
        >,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Send", (buffers, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_IList_1_SocketFlags_ByRefMut4(
        &mut self,
        buffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::System::ArraySegment_1<u8>,
            >,
        >,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Send", (buffers, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_i32_i32_SocketFlags1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Send", (buffer, offset, _cordl_size, socketFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_i32_i32_SocketFlags_ByRefMut3(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Send", (buffer, offset, _cordl_size, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_ReadOnlySpan_1_SocketFlags_ByRefMut2(
        &mut self,
        buffer: crate::System::ReadOnlySpan_1<u8>,
        socketFlags: crate::System::Net::Sockets::SocketFlags,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Send", (buffer, socketFlags, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_array_icall(
        sock: crate::System::IntPtr,
        bufarray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Send_array_icall",
                (sock, bufarray, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_icall(
        sock: crate::System::IntPtr,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Send_icall", (sock, buffer, count, flags, error, blocking))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_internal_SafeSocketHandle_Il2CppObject_i32_SocketFlags_ByRefMut__cordl_bool0(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        bufarray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Send_internal",
                (safeHandle, bufarray, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_internal_SafeSocketHandle_Il2CppObject_i32_SocketFlags_ByRefMut__cordl_bool1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::System::Net::Sockets::SocketFlags,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
        blocking: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Send_internal",
                (safeHandle, buffer, count, flags, error, blocking),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIPProtectionLevel(
        &mut self,
        level: crate::System::Net::Sockets::IPProtectionLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIPProtectionLevel", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSocketOption__cordl_bool1(
        &mut self,
        optionLevel: crate::System::Net::Sockets::SocketOptionLevel,
        optionName: crate::System::Net::Sockets::SocketOptionName,
        optionValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSocketOption", (optionLevel, optionName, optionValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSocketOption_i32_2(
        &mut self,
        optionLevel: crate::System::Net::Sockets::SocketOptionLevel,
        optionName: crate::System::Net::Sockets::SocketOptionName,
        optionValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSocketOption", (optionLevel, optionName, optionValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSocketOption_i32__cordl_bool0(
        &mut self,
        optionLevel: crate::System::Net::Sockets::SocketOptionLevel,
        optionName: crate::System::Net::Sockets::SocketOptionName,
        optionValue: i32,
        silent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSocketOption", (optionLevel, optionName, optionValue, silent))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSocketOption_icall(
        socket: crate::System::IntPtr,
        level: crate::System::Net::Sockets::SocketOptionLevel,
        name: crate::System::Net::Sockets::SocketOptionName,
        obj_val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byte_val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        int_val: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetSocketOption_icall",
                (socket, level, name, obj_val, byte_val, int_val, error),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSocketOption_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        level: crate::System::Net::Sockets::SocketOptionLevel,
        name: crate::System::Net::Sockets::SocketOptionName,
        obj_val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byte_val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        int_val: i32,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetSocketOption_internal",
                (safeHandle, level, name, obj_val, byte_val, int_val, error),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown(
        &mut self,
        how: crate::System::Net::Sockets::SocketShutdown,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Shutdown", (how))?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown_icall(
        socket: crate::System::IntPtr,
        how: crate::System::Net::Sockets::SocketShutdown,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shutdown_icall", (socket, how, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown_internal(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
        how: crate::System::Net::Sockets::SocketShutdown,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shutdown_internal", (safeHandle, how, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn SocketDefaults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SocketDefaults", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SocketOperationToSocketAsyncOperation(
        &mut self,
        op: crate::System::Net::Sockets::SocketOperation,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::Sockets::SocketAsyncOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::SocketAsyncOperation = __cordl_object
            .invoke("SocketOperationToSocketAsyncOperation", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn Socket_icall(
        family: crate::System::Net::Sockets::AddressFamily,
        _cordl_type: crate::System::Net::Sockets::SocketType,
        proto: crate::System::Net::Sockets::ProtocolType,
        error: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Socket_icall", (family, _cordl_type, proto, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfBufferNull(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfBufferNull", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfBufferOutOfRange(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfBufferOutOfRange", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfDisposedAndClosed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfDisposedAndClosed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfUdp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfUdp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateEndIAsyncResult(
        &mut self,
        ares: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncResult,
        > = __cordl_object
            .invoke("ValidateEndIAsyncResult", (ares, methodName, argName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AddressFamily_SocketType_ProtocolType0(
        &mut self,
        addressFamily: crate::System::Net::Sockets::AddressFamily,
        socketType: crate::System::Net::Sockets::SocketType,
        protocolType: crate::System::Net::Sockets::ProtocolType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (addressFamily, socketType, protocolType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SafeSocketHandle1(
        &mut self,
        family: crate::System::Net::Sockets::AddressFamily,
        _cordl_type: crate::System::Net::Sockets::SocketType,
        proto: crate::System::Net::Sockets::ProtocolType,
        safe_handle: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SafeSocketHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (family, _cordl_type, proto, safe_handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn cancel_blocking_socket_operation(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cancel_blocking_socket_operation", (thread))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AddressFamily(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::AddressFamily> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::AddressFamily = __cordl_object
            .invoke("get_AddressFamily", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Available(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Available", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Blocking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Blocking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CleanedUp(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CleanedUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Connected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Connected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DualMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DualMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FamilyHint() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_FamilyHint", ())?;
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
    pub fn get_InternalSyncObject() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalSyncObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBound(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsBound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDualMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDualMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint> = __cordl_object
            .invoke("get_LocalEndPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OSSupportsIPv4() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OSSupportsIPv4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OSSupportsIPv6() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OSSupportsIPv6", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProtocolType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::ProtocolType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::ProtocolType = __cordl_object
            .invoke("get_ProtocolType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint> = __cordl_object
            .invoke("get_RemoteEndPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SocketType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::SocketType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::SocketType = __cordl_object
            .invoke("get_SocketType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Ttl(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("get_Ttl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Blocking(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Blocking", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DontFragment(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DontFragment", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DualMode(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DualMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EnableBroadcast(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnableBroadcast", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExclusiveAddressUse(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExclusiveAddressUse", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NoDelay(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NoDelay", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReceiveBufferSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReceiveBufferSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReceiveTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReceiveTimeout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SendBufferSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendBufferSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SendTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendTimeout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Ttl(
        &mut self,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Ttl", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+Socket")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Sockets::Socket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Sockets+Socket")]
impl AsRef<crate::System::IDisposable> for crate::System::Net::Sockets::Socket {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket")]
impl AsMut<crate::System::IDisposable> for crate::System::Net::Sockets::Socket {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct Socket_AwaitableSocketAsyncEventArgs {
    __cordl_parent: crate::System::Net::Sockets::SocketAsyncEventArgs,
    pub _continuation: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _executionContext: *mut crate::System::Threading::ExecutionContext,
    pub _scheduler: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _token: i16,
    pub _WrapExceptionsInIOExceptions_k__BackingField: bool,
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs => "System.Net.Sockets"
    ."Socket/AwaitableSocketAsyncEventArgs"
);
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl std::ops::Deref
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    type Target = crate::System::Net::Sockets::SocketAsyncEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl std::ops::DerefMut
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    pub fn CreateException(
        &mut self,
        error: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("CreateException", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResult(&mut self, token: i16) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetResult", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStatus(
        &mut self,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus = __cordl_object
            .invoke("GetStatus", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeContinuation(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        forceAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeContinuation", (continuation, state, forceAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCompleted_Action_1_Il2CppObject_i16_ValueTaskSourceOnCompletedFlags1(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        token: i16,
        flags: crate::System::Threading::Tasks::Sources::ValueTaskSourceOnCompletedFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", (continuation, state, token, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCompleted_SocketAsyncEventArgs0(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveAsync(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ValueTask_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask_1<i32> = __cordl_object
            .invoke("ReceiveAsync", (socket))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reserve(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Reserve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncForNetworkStream(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("SendAsyncForNetworkStream", (socket))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Threading_Tasks_Sources_IValueTaskSource_GetResult(
        &mut self,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Threading.Tasks.Sources.IValueTaskSource.GetResult",
                (token),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowException(
        &mut self,
        error: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowException", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIncorrectTokenException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIncorrectTokenException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowMultipleContinuationsException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowMultipleContinuationsException", ())?;
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
    pub fn get_WrapExceptionsInIOExceptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_WrapExceptionsInIOExceptions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WrapExceptionsInIOExceptions(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WrapExceptionsInIOExceptions", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl AsRef<crate::System::Threading::Tasks::Sources::IValueTaskSource>
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::Sources::IValueTaskSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl AsMut<crate::System::Threading::Tasks::Sources::IValueTaskSource>
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Threading::Tasks::Sources::IValueTaskSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl AsRef<crate::System::Threading::Tasks::Sources::IValueTaskSource_1<i32>>
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn as_ref(
        &self,
    ) -> &crate::System::Threading::Tasks::Sources::IValueTaskSource_1<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+AwaitableSocketAsyncEventArgs")]
impl AsMut<crate::System::Threading::Tasks::Sources::IValueTaskSource_1<i32>>
for crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Threading::Tasks::Sources::IValueTaskSource_1<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct Socket_CachedEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TaskAccept: *mut crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<
        *mut crate::System::Net::Sockets::Socket,
    >,
    pub TaskReceive: *mut crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs,
    pub TaskSend: *mut crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs,
    pub ValueTaskReceive: *mut crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs,
    pub ValueTaskSend: *mut crate::System::Net::Sockets::Socket_AwaitableSocketAsyncEventArgs,
}
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::Socket_CachedEventArgs =>
    "System.Net.Sockets"."Socket/CachedEventArgs"
);
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
impl std::ops::Deref for crate::System::Net::Sockets::Socket_CachedEventArgs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
impl std::ops::DerefMut for crate::System::Net::Sockets::Socket_CachedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
impl crate::System::Net::Sockets::Socket_CachedEventArgs {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "System+Net+Sockets+Socket+CachedEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::Socket_CachedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct Socket_Int32TaskSocketAsyncEventArgs {
    __cordl_parent: crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<i32>,
    pub _wrapExceptionsInIOExceptions: bool,
}
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs => "System.Net.Sockets"
    ."Socket/Int32TaskSocketAsyncEventArgs"
);
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
impl std::ops::Deref
for crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs {
    type Target = crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
impl std::ops::DerefMut
for crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
impl crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "System+Net+Sockets+Socket+Int32TaskSocketAsyncEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::Socket_Int32TaskSocketAsyncEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Socket_TaskSocketAsyncEventArgs_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Net::Sockets::SocketAsyncEventArgs,
    pub _builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        TResult,
    >,
    pub _accessed: bool,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1 < TResult > =>
    "System.Net.Sockets"."Socket/TaskSocketAsyncEventArgs`1" < TResult >
);
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<TResult> {
    type Target = crate::System::Net::Sockets::SocketAsyncEventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<TResult> {
    pub fn GetCompletionResponsibility(
        &mut self,
        responsibleForReturningToPool: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
            TResult,
        > = __cordl_object
            .invoke("GetCompletionResponsibility", (responsibleForReturningToPool))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+TaskSocketAsyncEventArgs_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::Socket_TaskSocketAsyncEventArgs_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+WSABUF")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Socket_WSABUF {
    pub len: i32,
    pub buf: crate::System::IntPtr,
}
#[cfg(feature = "System+Net+Sockets+Socket+WSABUF")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::Socket_WSABUF =>
    "System.Net.Sockets"."Socket/WSABUF"
);
#[cfg(feature = "System+Net+Sockets+Socket+WSABUF")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::Sockets::Socket_WSABUF {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+Sockets+Socket+WSABUF")]
impl crate::System::Net::Sockets::Socket_WSABUF {}
