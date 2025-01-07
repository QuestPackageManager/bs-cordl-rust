#[cfg(feature = "System+Net+ServicePoint")]
#[repr(C)]
#[derive(Debug)]
pub struct ServicePoint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub lastDnsResolve: crate::System::DateTime,
    pub protocolVersion: quest_hook::libil2cpp::Gc<crate::System::Version>,
    pub host: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    pub usesProxy: bool,
    pub sendContinue: bool,
    pub useConnect: bool,
    pub hostE: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub useNagle: bool,
    pub endPointCallback: quest_hook::libil2cpp::Gc<crate::System::Net::BindIPEndPoint>,
    pub tcp_keepalive: bool,
    pub tcp_keepalive_time: i32,
    pub tcp_keepalive_interval: i32,
    pub disposed: bool,
    pub connectionLeaseTimeout: i32,
    pub receiveBufferSize: i32,
    pub _Key_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::ServicePointManager_SPKey,
    >,
    pub _Scheduler_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::ServicePointScheduler,
    >,
    pub connectionLimit: i32,
    pub maxIdleTime: i32,
    pub m_ServerCertificateOrBytes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub m_ClientCertificateOrBytes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "System+Net+ServicePoint")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::ServicePoint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "ServicePoint";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+ServicePoint")]
impl std::ops::Deref for crate::System::Net::ServicePoint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        sock: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        remote: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CallEndPointDelegate", (sock, remote))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseConnectionGroup(
        &mut self,
        connectionGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CloseConnectionGroup", (connectionGroupName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeServicePoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeServicePoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KeepAliveSetup(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeepAliveSetup", (socket))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        key: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointManager_SPKey>,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, uri, connectionLimit, maxIdleTime))?;
        Ok(__cordl_object.into())
    }
    pub fn PutBytes(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        v: u32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutBytes", (bytes, v, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendRequest(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::System::Net::WebOperation>,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRequest", (operation, groupName))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetVersion(
        &mut self,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVersion", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClientCertificate(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClientCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateServerCertificate(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateServerCertificate", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointManager_SPKey>,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        connectionLimit: i32,
        maxIdleTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, uri, connectionLimit, maxIdleTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectionLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConnectionLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasTimedOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasTimedOut", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HostEntry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPHostEntry> = __cordl_object
            .invoke("get_HostEntry", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointManager_SPKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointManager_SPKey,
        > = __cordl_object.invoke("get_Key", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProtocolVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = __cordl_object
            .invoke("get_ProtocolVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointScheduler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServicePointScheduler,
        > = __cordl_object.invoke("get_Scheduler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SendContinue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SendContinue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseConnect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseConnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseNagleAlgorithm(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseNagleAlgorithm", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UsesProxy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UsesProxy", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_Scheduler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePointScheduler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Scheduler", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
