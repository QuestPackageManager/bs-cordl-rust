#[cfg(feature = "System+Net+HttpListenerResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListenerResponse {
    __cordl_parent: crate::System::Object,
    pub disposed: bool,
    pub content_encoding: *mut crate::System::Text::Encoding,
    pub content_length: i64,
    pub cl_set: bool,
    pub content_type: *mut crate::System::String,
    pub cookies: *mut crate::System::Net::CookieCollection,
    pub headers: *mut crate::System::Net::WebHeaderCollection,
    pub keep_alive: bool,
    pub output_stream: *mut crate::System::Net::ResponseStream,
    pub version: *mut crate::System::Version,
    pub location: *mut crate::System::String,
    pub status_code: i32,
    pub status_description: *mut crate::System::String,
    pub chunked: bool,
    pub context: *mut crate::System::Net::HttpListenerContext,
    pub HeadersSent: bool,
    pub headers_lock: *mut crate::System::Object,
    pub force_close_chunked: bool,
}
#[cfg(feature = "System+Net+HttpListenerResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListenerResponse =>
    "System.Net"."HttpListenerResponse"
);
#[cfg(feature = "System+Net+HttpListenerResponse")]
impl std::ops::Deref for crate::System::Net::HttpListenerResponse {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerResponse")]
impl std::ops::DerefMut for crate::System::Net::HttpListenerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerResponse")]
impl crate::System::Net::HttpListenerResponse {
    pub fn get_ForceCloseChunked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ForceCloseChunked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_OutputStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_SendChunked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendChunked", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_StatusCode(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StatusCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Close__cordl_bool0(
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
    pub fn Close_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close_Il2CppArray__cordl_bool2(
        &mut self,
        responseEntity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        willBlock: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (responseEntity, willBlock))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentEncoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("get_ContentEncoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ContentLength64(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentLength64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_SendChunked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SendChunked", ())?;
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
    pub fn set_ContentType(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendHeaders(
        &mut self,
        closing: bool,
        ms: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendHeaders", (closing, ms))?;
        Ok(__cordl_ret)
    }
    pub fn get_Headers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebHeaderCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebHeaderCollection = __cordl_object
            .invoke("get_Headers", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: *mut crate::System::Net::HttpListenerContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+HttpListenerResponse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpListenerResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
