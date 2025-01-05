#[cfg(feature = "System+Net+FtpWebResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebResponse {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>,
    pub _responseStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _contentLength: i64,
    pub _responseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub _statusCode: crate::System::Net::FtpStatusCode,
    pub _statusLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _ftpRequestHeaders: quest_hook::libil2cpp::Gc<
        crate::System::Net::WebHeaderCollection,
    >,
    pub _lastModified: crate::System::DateTime,
    pub _bannerMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _welcomeMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _exitMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+FtpWebResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebResponse => "System.Net"
    ."FtpWebResponse"
);
#[cfg(feature = "System+Net+FtpWebResponse")]
impl std::ops::Deref for crate::System::Net::FtpWebResponse {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Net::WebResponse>;
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
        Ok(__cordl_ret.into())
    }
    pub fn GetResponseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetResponseStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        responseStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        contentLength: i64,
        responseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lastModified: crate::System::DateTime,
        bannerMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        welcomeMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exitMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn SetResponseStream(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetResponseStream", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateStatus(
        &mut self,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exitMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStatus", (statusCode, statusLine, exitMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        responseStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        contentLength: i64,
        responseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        statusCode: crate::System::Net::FtpStatusCode,
        statusLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lastModified: crate::System::DateTime,
        bannerMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        welcomeMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exitMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Headers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::WebHeaderCollection,
        > = __cordl_object.invoke("get_Headers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_ResponseUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StatusCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::FtpStatusCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::FtpStatusCode = __cordl_object
            .invoke("get_StatusCode", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Net+FtpWebResponse")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Net::FtpWebResponse {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Net::FtpWebResponse {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpWebResponse_EmptyStream {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
}
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpWebResponse_EmptyStream =>
    "System.Net"."FtpWebResponse/EmptyStream"
);
#[cfg(feature = "System+Net+FtpWebResponse+EmptyStream")]
impl std::ops::Deref for crate::System::Net::FtpWebResponse_EmptyStream {
    type Target = quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>;
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
