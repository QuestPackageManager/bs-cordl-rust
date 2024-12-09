#[cfg(feature = "System+Net+WebConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct WebConnection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ntlm_credentials: *mut crate::System::Net::NetworkCredential,
    pub ntlm_authenticated: bool,
    pub unsafe_sharing: bool,
    pub networkStream: *mut crate::System::IO::Stream,
    pub socket: *mut crate::System::Net::Sockets::Socket,
    pub monoTlsStream: *mut crate::Mono::Net::Security::MonoTlsStream,
    pub tunnel: *mut crate::System::Net::WebConnectionTunnel,
    pub disposed: i32,
    pub _ServicePoint_k__BackingField: *mut crate::System::Net::ServicePoint,
    pub idleSince: crate::System::DateTime,
    pub currentOperation: *mut crate::System::Net::WebOperation,
}
#[cfg(feature = "System+Net+WebConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebConnection => "System.Net"
    ."WebConnection"
);
#[cfg(feature = "System+Net+WebConnection")]
impl std::ops::Deref for crate::System::Net::WebConnection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebConnection")]
impl std::ops::DerefMut for crate::System::Net::WebConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebConnection")]
impl crate::System::Net::WebConnection {
    #[cfg(feature = "System+Net+WebConnection+_Connect_d__16")]
    pub type _Connect_d__16 = crate::System::Net::WebConnection__Connect_d__16;
    #[cfg(feature = "System+Net+WebConnection+_CreateStream_d__18")]
    pub type _CreateStream_d__18 = crate::System::Net::WebConnection__CreateStream_d__18;
    #[cfg(feature = "System+Net+WebConnection+_InitConnection_d__19")]
    pub type _InitConnection_d__19 = crate::System::Net::WebConnection__InitConnection_d__19;
    #[cfg(feature = "System+Net+WebConnection+__c")]
    pub type __c = crate::System::Net::WebConnection___c;
    pub fn CanReuse(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanReuse", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanReuseConnection(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanReuseConnection", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn CheckReusable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckReusable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
        reset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (reset))?;
        Ok(__cordl_ret)
    }
    pub fn CloseSocket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseSocket", ())?;
        Ok(__cordl_ret)
    }
    pub fn Connect(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("Connect", (operation, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Continue(
        &mut self,
        next: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Continue", (next))?;
        Ok(__cordl_ret)
    }
    pub fn CreateStream(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        reused: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("CreateStream", (operation, reused, cancellationToken))?;
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
    pub fn InitConnection(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Net::WebRequestStream,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Net::WebRequestStream,
        > = __cordl_object.invoke("InitConnection", (operation, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sPoint: *mut crate::System::Net::ServicePoint,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sPoint))?;
        Ok(__cordl_object)
    }
    pub fn PrepareSharingNtlm(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PrepareSharingNtlm", (operation))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetNtlm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetNtlm", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartOperation(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        reused: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartOperation", (operation, reused))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sPoint: *mut crate::System::Net::ServicePoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sPoint))?;
        Ok(__cordl_ret)
    }
    pub fn get_Closed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Closed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IdleSince(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_IdleSince", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NtlmAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_NtlmAuthenticated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NtlmCredential(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::NetworkCredential> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkCredential = __cordl_object
            .invoke("get_NtlmCredential", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ServicePoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePoint = __cordl_object
            .invoke("get_ServicePoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnsafeAuthenticatedConnectionSharing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_UnsafeAuthenticatedConnectionSharing", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_NtlmAuthenticated(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NtlmAuthenticated", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_NtlmCredential(
        &mut self,
        value: *mut crate::System::Net::NetworkCredential,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NtlmCredential", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UnsafeAuthenticatedConnectionSharing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UnsafeAuthenticatedConnectionSharing", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebConnection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
