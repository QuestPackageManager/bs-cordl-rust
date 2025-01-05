#[cfg(feature = "System+Net+Sockets+TcpListener")]
#[repr(C)]
#[derive(Debug)]
pub struct TcpListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ServerSocketEP: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub m_ServerSocket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub m_Active: bool,
    pub m_ExclusiveAddressUse: bool,
}
#[cfg(feature = "System+Net+Sockets+TcpListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::TcpListener =>
    "System.Net.Sockets"."TcpListener"
);
#[cfg(feature = "System+Net+Sockets+TcpListener")]
impl std::ops::Deref for crate::System::Net::Sockets::TcpListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+TcpListener")]
impl std::ops::DerefMut for crate::System::Net::Sockets::TcpListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+TcpListener")]
impl crate::System::Net::Sockets::TcpListener {
    pub fn BeginAcceptTcpClient(
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
            .invoke("BeginAcceptTcpClient", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndAcceptTcpClient(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::TcpClient,
        > = __cordl_object.invoke("EndAcceptTcpClient", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        localaddr: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localaddr, port))?;
        Ok(__cordl_object.into())
    }
    pub fn Start_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_i32_1(
        &mut self,
        backlog: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (backlog))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        localaddr: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localaddr, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalEndpoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::EndPoint> = __cordl_object
            .invoke("get_LocalEndpoint", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Sockets+TcpListener")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Sockets::TcpListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
