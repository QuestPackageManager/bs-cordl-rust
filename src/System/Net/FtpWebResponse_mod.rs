#[cfg(feature = "System+Net+FtpWebResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebResponse {
    __cordl_parent: crate::System::Net::WebResponse,
    pub _responseStream: *mut crate::System::IO::Stream,
    pub _contentLength: i64,
    pub _responseUri: *mut crate::System::Uri,
    pub _statusCode: crate::System::Net::FtpStatusCode,
    pub _statusLine: *mut crate::System::String,
    pub _ftpRequestHeaders: *mut crate::System::Net::WebHeaderCollection,
    pub _lastModified: crate::System::DateTime,
    pub _bannerMessage: *mut crate::System::String,
    pub _welcomeMessage: *mut crate::System::String,
    pub _exitMessage: *mut crate::System::String,
}
#[cfg(feature = "System+Net+FtpWebResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebResponse => "System.Net"
    ."FtpWebResponse"
);
#[cfg(feature = "System+Net+FtpWebResponse")]
impl std::ops::Deref for crate::System::Net::FtpWebResponse {
    type Target = crate::System::Net::WebResponse;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse")]
impl std::ops::DerefMut for crate::System::Net::FtpWebResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse")]
impl crate::System::Net::FtpWebResponse {
    #[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
    pub type EmptyStream = crate::System::Net::FtpWebResponse_EmptyStream;
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
    pub fn GetResponseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetResponseStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        responseStream: *mut crate::System::IO::Stream,
        contentLength: i64,
        responseUri: *mut crate::System::Uri,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: *mut crate::System::String,
        lastModified: crate::System::DateTime,
        bannerMessage: *mut crate::System::String,
        welcomeMessage: *mut crate::System::String,
        exitMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    responseStream,
                    contentLength,
                    responseUri,
                    statusCode,
                    statusLine,
                    lastModified,
                    bannerMessage,
                    welcomeMessage,
                    exitMessage,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetResponseStream(
        &mut self,
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetResponseStream", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStatus(
        &mut self,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: *mut crate::System::String,
        exitMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStatus", (statusCode, statusLine, exitMessage))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        responseStream: *mut crate::System::IO::Stream,
        contentLength: i64,
        responseUri: *mut crate::System::Uri,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: *mut crate::System::String,
        lastModified: crate::System::DateTime,
        bannerMessage: *mut crate::System::String,
        welcomeMessage: *mut crate::System::String,
        exitMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    responseStream,
                    contentLength,
                    responseUri,
                    statusCode,
                    statusLine,
                    lastModified,
                    bannerMessage,
                    welcomeMessage,
                    exitMessage,
                ),
            )?;
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
    pub fn get_ResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_ResponseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StatusCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::FtpStatusCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::FtpStatusCode = __cordl_object
            .invoke("get_StatusCode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+FtpWebResponse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpWebResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebResponse_EmptyStream {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebResponse_EmptyStream =>
    "System.Net"."FtpWebResponse/EmptyStream"
);
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
impl std::ops::Deref for crate::System::Net::FtpWebResponse_EmptyStream {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
impl std::ops::DerefMut for crate::System::Net::FtpWebResponse_EmptyStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
impl crate::System::Net::FtpWebResponse_EmptyStream {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::FtpWebResponse_EmptyStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
