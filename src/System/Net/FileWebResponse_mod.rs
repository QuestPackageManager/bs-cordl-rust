#[cfg(feature = "System+Net+FileWebResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct FileWebResponse {
    __cordl_parent: crate::System::Net::WebResponse,
    pub m_closed: bool,
    pub m_contentLength: i64,
    pub m_fileAccess: crate::System::IO::FileAccess,
    pub m_headers: *mut crate::System::Net::WebHeaderCollection,
    pub m_stream: *mut crate::System::IO::Stream,
    pub m_uri: *mut crate::System::Uri,
}
#[cfg(feature = "System+Net+FileWebResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FileWebResponse => "System.Net"
    ."FileWebResponse"
);
#[cfg(feature = "System+Net+FileWebResponse")]
impl std::ops::Deref for crate::System::Net::FileWebResponse {
    type Target = crate::System::Net::WebResponse;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebResponse")]
impl std::ops::DerefMut for crate::System::Net::FileWebResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebResponse")]
impl crate::System::Net::FileWebResponse {
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
    pub fn GetObjectData(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (serializationInfo, streamingContext))?;
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
    pub fn New_FileWebRequest_Uri_FileAccess__cordl_bool0(
        request: *mut crate::System::Net::FileWebRequest,
        uri: *mut crate::System::Uri,
        access: crate::System::IO::FileAccess,
        asyncHint: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, uri, access, asyncHint))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext1(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn System_Net_ICloseEx_CloseEx(
        &mut self,
        closeState: crate::System::Net::CloseExState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Net.ICloseEx.CloseEx", (closeState))?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (serializationInfo, streamingContext),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_FileWebRequest_Uri_FileAccess__cordl_bool0(
        &mut self,
        request: *mut crate::System::Net::FileWebRequest,
        uri: *mut crate::System::Uri,
        access: crate::System::IO::FileAccess,
        asyncHint: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, uri, access, asyncHint))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationInfo, streamingContext))?;
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
}
#[cfg(feature = "System+Net+FileWebResponse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FileWebResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}