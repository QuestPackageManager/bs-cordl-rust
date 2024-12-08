#[cfg(feature = "System+Net+EndPointListener")]
#[repr(C)]
#[derive(Debug)]
pub struct EndPointListener {
    __cordl_parent: crate::System::Object,
    pub listener: *mut crate::System::Net::HttpListener,
    pub endpoint: *mut crate::System::Net::IPEndPoint,
    pub sock: *mut crate::System::Net::Sockets::Socket,
    pub prefixes: *mut crate::System::Collections::Hashtable,
    pub unhandled: *mut crate::System::Collections::ArrayList,
    pub all: *mut crate::System::Collections::ArrayList,
    pub cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    pub secure: bool,
    pub unregistered: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Net::HttpConnection,
        *mut crate::System::Net::HttpConnection,
    >,
}
#[cfg(feature = "System+Net+EndPointListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::EndPointListener => "System.Net"
    ."EndPointListener"
);
#[cfg(feature = "System+Net+EndPointListener")]
impl std::ops::Deref for crate::System::Net::EndPointListener {
    type Target = crate::System::Object;
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
    pub fn AddPrefix(
        &mut self,
        prefix: *mut crate::System::Net::ListenerPrefix,
        listener: *mut crate::System::Net::HttpListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPrefix", (prefix, listener))?;
        Ok(__cordl_ret)
    }
    pub fn AddSpecial(
        &mut self,
        coll: *mut crate::System::Collections::ArrayList,
        prefix: *mut crate::System::Net::ListenerPrefix,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpecial", (coll, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn BindContext(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BindContext", (context))?;
        Ok(__cordl_ret)
    }
    pub fn CheckIfRemove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfRemove", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchFromList(
        &mut self,
        host: *mut crate::System::String,
        path: *mut crate::System::String,
        list: *mut crate::System::Collections::ArrayList,
        prefix: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::ListenerPrefix>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListener> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListener = __cordl_object
            .invoke("MatchFromList", (host, path, list, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        listener: *mut crate::System::Net::HttpListener,
        addr: *mut crate::System::Net::IPAddress,
        port: i32,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listener, addr, port, secure))?;
        Ok(__cordl_object)
    }
    pub fn RemoveConnection(
        &mut self,
        conn: *mut crate::System::Net::HttpConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (conn))?;
        Ok(__cordl_ret)
    }
    pub fn RemovePrefix(
        &mut self,
        prefix: *mut crate::System::Net::ListenerPrefix,
        listener: *mut crate::System::Net::HttpListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePrefix", (prefix, listener))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSpecial(
        &mut self,
        coll: *mut crate::System::Collections::ArrayList,
        prefix: *mut crate::System::Net::ListenerPrefix,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveSpecial", (coll, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn SearchListener(
        &mut self,
        uri: *mut crate::System::Uri,
        prefix: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::ListenerPrefix>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListener> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListener = __cordl_object
            .invoke("SearchListener", (uri, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindContext(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindContext", (context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        listener: *mut crate::System::Net::HttpListener,
        addr: *mut crate::System::Net::IPAddress,
        port: i32,
        secure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listener, addr, port, secure))?;
        Ok(__cordl_ret)
    }
    pub fn get_Listener(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListener> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListener = __cordl_object
            .invoke("get_Listener", ())?;
        Ok(__cordl_ret)
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
