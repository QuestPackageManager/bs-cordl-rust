#[cfg(feature = "System+Net+HttpListener")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tlsProvider: *mut crate::Mono::Security::Interface::MonoTlsProvider,
    pub tlsSettings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    pub certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    pub auth_schemes: crate::System::Net::AuthenticationSchemes,
    pub prefixes: *mut crate::System::Net::HttpListenerPrefixCollection,
    pub auth_selector: *mut crate::System::Net::AuthenticationSchemeSelector,
    pub realm: *mut quest_hook::libil2cpp::Il2CppString,
    pub ignore_write_exceptions: bool,
    pub listening: bool,
    pub disposed: bool,
    pub _internalLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub registry: *mut crate::System::Collections::Hashtable,
    pub ctx_queue: *mut crate::System::Collections::ArrayList,
    pub wait_queue: *mut crate::System::Collections::ArrayList,
    pub connections: *mut crate::System::Collections::Hashtable,
    pub defaultServiceNames: *mut crate::System::Net::ServiceNameStore,
    pub extendedProtectionPolicy: *mut crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicy,
}
#[cfg(feature = "System+Net+HttpListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListener => "System.Net"
    ."HttpListener"
);
#[cfg(feature = "System+Net+HttpListener")]
impl std::ops::Deref for crate::System::Net::HttpListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListener")]
impl std::ops::DerefMut for crate::System::Net::HttpListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListener")]
impl crate::System::Net::HttpListener {
    pub fn AddConnection(
        &mut self,
        cnc: *mut crate::System::Net::HttpConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddConnection", (cnc))?;
        Ok(__cordl_ret)
    }
    pub fn BeginGetContext(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        state: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginGetContext", (callback, state))?;
        Ok(__cordl_ret)
    }
    pub fn CheckDisposed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckDisposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn Cleanup(
        &mut self,
        close_existing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", (close_existing))?;
        Ok(__cordl_ret)
    }
    pub fn Close_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close__cordl_bool1(
        &mut self,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (force))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSslStream(
        &mut self,
        innerStream: *mut crate::System::IO::Stream,
        ownsStream: bool,
        callback: *mut crate::System::Net::Security::RemoteCertificateValidationCallback,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Security::SslStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Security::SslStream = __cordl_object
            .invoke("CreateSslStream", (innerStream, ownsStream, callback))?;
        Ok(__cordl_ret)
    }
    pub fn EndGetContext(
        &mut self,
        asyncResult: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListenerContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListenerContext = __cordl_object
            .invoke("EndGetContext", (asyncResult))?;
        Ok(__cordl_ret)
    }
    pub fn GetContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListenerContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListenerContext = __cordl_object
            .invoke("GetContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetContextFromQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::HttpListenerContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListenerContext = __cordl_object
            .invoke("GetContextFromQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadCertificateAndKey(
        &mut self,
        addr: *mut crate::System::Net::IPAddress,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("LoadCertificateAndKey", (addr, port))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterContext(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterContext", (context))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveConnection(
        &mut self,
        cnc: *mut crate::System::Net::HttpConnection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", (cnc))?;
        Ok(__cordl_ret)
    }
    pub fn SelectAuthenticationScheme(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::AuthenticationSchemes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::AuthenticationSchemes = __cordl_object
            .invoke("SelectAuthenticationScheme", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterContext(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterContext", (context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AuthenticationSchemeSelectorDelegate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::AuthenticationSchemeSelector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::AuthenticationSchemeSelector = __cordl_object
            .invoke("get_AuthenticationSchemeSelectorDelegate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AuthenticationSchemes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::AuthenticationSchemes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::AuthenticationSchemes = __cordl_object
            .invoke("get_AuthenticationSchemes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IgnoreWriteExceptions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreWriteExceptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsListening(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsListening", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefixes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::HttpListenerPrefixCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::HttpListenerPrefixCollection = __cordl_object
            .invoke("get_Prefixes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Realm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Realm", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+HttpListener")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
