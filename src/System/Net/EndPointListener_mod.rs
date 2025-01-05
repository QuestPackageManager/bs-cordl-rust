#[cfg(feature = "System+Net+EndPointListener")]
#[repr(C)]
#[derive(Debug)]
pub struct EndPointListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    pub endpoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub sock: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub prefixes: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub unhandled: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub all: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub cert: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    >,
    pub secure: bool,
    pub unregistered: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
            quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
        >,
    >,
}
#[cfg(feature = "System+Net+EndPointListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::EndPointListener => "System.Net"
    ."EndPointListener"
);
#[cfg(feature = "System+Net+EndPointListener")]
impl std::ops::Deref for crate::System::Net::EndPointListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointListener")]
impl std::ops::DerefMut for crate::System::Net::EndPointListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointListener")]
impl crate::System::Net::EndPointListener {
    pub fn Accept(
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
        accepted: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Accept", (socket, e, accepted))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPrefix(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPrefix", (prefix, listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpecial(
        &mut self,
        coll: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        prefix: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpecial", (coll, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindContext(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BindContext", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfRemove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfRemove", ())?;
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
    pub fn MatchFromList(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener> = __cordl_object
            .invoke("MatchFromList", (host, path, list, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
        addr: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listener, addr, port, secure))?;
        Ok(__cordl_object.into())
    }
    pub fn OnAccept(
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketAsyncEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnAccept", (sender, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessAccept(
        args: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::SocketAsyncEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessAccept", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveConnection(
        &mut self,
        conn: quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (conn))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePrefix(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePrefix", (prefix, listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSpecial(
        &mut self,
        coll: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        prefix: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveSpecial", (coll, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchListener(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        prefix: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener> = __cordl_object
            .invoke("SearchListener", (uri, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnbindContext(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindContext", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
        addr: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listener, addr, port, secure))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Listener(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener> = __cordl_object
            .invoke("get_Listener", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+EndPointListener")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::EndPointListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
