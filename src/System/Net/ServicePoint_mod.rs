#[cfg(feature = "System+Net+ServicePoint")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePoint {
    __cordl_parent: crate::System::Object,
    pub uri: *mut crate::System::Uri,
    pub lastDnsResolve: crate::System::DateTime,
    pub protocolVersion: *mut crate::System::Version,
    pub host: *mut crate::System::Net::IPHostEntry,
    pub usesProxy: bool,
    pub sendContinue: bool,
    pub useConnect: bool,
    pub hostE: *mut crate::System::Object,
    pub useNagle: bool,
    pub endPointCallback: *mut crate::System::Net::BindIPEndPoint,
    pub tcp_keepalive: bool,
    pub tcp_keepalive_time: i32,
    pub tcp_keepalive_interval: i32,
    pub disposed: bool,
    pub connectionLeaseTimeout: i32,
    pub receiveBufferSize: i32,
    pub _Key_k__BackingField: *mut crate::System::Net::ServicePointManager_SPKey,
    pub _Scheduler_k__BackingField: *mut crate::System::Net::ServicePointScheduler,
    pub connectionLimit: i32,
    pub maxIdleTime: i32,
    pub m_ServerCertificateOrBytes: *mut crate::System::Object,
    pub m_ClientCertificateOrBytes: *mut crate::System::Object,
}
#[cfg(feature = "System+Net+ServicePoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ServicePoint => "System.Net"
    ."ServicePoint"
);
#[cfg(feature = "System+Net+ServicePoint")]
impl std::ops::Deref for crate::System::Net::ServicePoint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePoint")]
impl std::ops::DerefMut for crate::System::Net::ServicePoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ServicePoint")]
impl crate::System::Net::ServicePoint {
    pub fn CallEndPointDelegate(
        &mut self,
        sock: *mut crate::System::Net::Sockets::Socket,
        remote: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CallEndPointDelegate", (sock, remote))?;
        Ok(__cordl_ret)
    }
    pub fn CloseConnectionGroup(
        &mut self,
        connectionGroupName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CloseConnectionGroup", (connectionGroupName))?;
        Ok(__cordl_ret)
    }
    pub fn FreeServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeServicePoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn KeepAliveSetup(
        &mut self,
        socket: *mut crate::System::Net::Sockets::Socket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeepAliveSetup", (socket))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        key: *mut crate::System::Net::ServicePointManager_SPKey,
        uri: *mut crate::System::Uri,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, uri, connectionLimit, maxIdleTime))?;
        Ok(__cordl_object)
    }
    pub fn SendRequest(
        &mut self,
        operation: *mut crate::System::Net::WebOperation,
        groupName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRequest", (operation, groupName))?;
        Ok(__cordl_ret)
    }
    pub fn SetTcpKeepAlive(
        &mut self,
        enabled: bool,
        keepAliveTime: i32,
        keepAliveInterval: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTcpKeepAlive", (enabled, keepAliveTime, keepAliveInterval))?;
        Ok(__cordl_ret)
    }
    pub fn SetVersion(
        &mut self,
        version: *mut crate::System::Version,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVersion", (version))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateClientCertificate(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClientCertificate", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateServerCertificate(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateServerCertificate", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        key: *mut crate::System::Net::ServicePointManager_SPKey,
        uri: *mut crate::System::Uri,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, uri, connectionLimit, maxIdleTime))?;
        Ok(__cordl_ret)
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConnectionLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConnectionLimit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasTimedOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasTimedOut", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HostEntry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPHostEntry> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPHostEntry = __cordl_object
            .invoke("get_HostEntry", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::ServicePointManager_SPKey,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePointManager_SPKey = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProtocolVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Version> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Version = __cordl_object
            .invoke("get_ProtocolVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Scheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ServicePointScheduler> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ServicePointScheduler = __cordl_object
            .invoke("get_Scheduler", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SendContinue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SendContinue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseConnect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseConnect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseNagleAlgorithm(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseNagleAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UsesProxy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UsesProxy", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Expect100Continue(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Expect100Continue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Scheduler(
        &mut self,
        value: *mut crate::System::Net::ServicePointScheduler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Scheduler", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SendContinue(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendContinue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UseConnect(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseConnect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UseNagleAlgorithm(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseNagleAlgorithm", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UsesProxy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UsesProxy", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+ServicePoint")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ServicePoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
