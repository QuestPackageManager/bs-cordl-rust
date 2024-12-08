#[cfg(feature = "System+Net+WebException")]
#[repr(C)]
#[derive(Debug)]
pub struct WebException {
    __cordl_parent: crate::System::InvalidOperationException,
    pub m_Status: crate::System::Net::WebExceptionStatus,
    pub m_Response: *mut crate::System::Net::WebResponse,
    pub m_InternalStatus: crate::System::Net::WebExceptionInternalStatus,
}
#[cfg(feature = "System+Net+WebException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebException => "System.Net"
    ."WebException"
);
#[cfg(feature = "System+Net+WebException")]
impl std::ops::Deref for crate::System::Net::WebException {
    type Target = crate::System::InvalidOperationException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebException")]
impl std::ops::DerefMut for crate::System::Net::WebException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebException")]
impl crate::System::Net::WebException {
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext9(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception2(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception_WebExceptionStatus_WebResponse5(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException, status, response))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception_WebExceptionStatus_WebResponse_WebExceptionInternalStatus7(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (message, innerException, status, response, internalStatus),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_Exception_WebExceptionStatus_WebResponse6(
        message: *mut crate::System::String,
        data: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, data, innerException, status, response))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_Exception_WebExceptionStatus_WebResponse_WebExceptionInternalStatus8(
        message: *mut crate::System::String,
        data: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (message, data, innerException, status, response, internalStatus),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_WebExceptionStatus3(
        message: *mut crate::System::String,
        status: crate::System::Net::WebExceptionStatus,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, status))?;
        Ok(__cordl_object)
    }
    pub fn New_String_WebExceptionStatus_WebExceptionInternalStatus_Exception4(
        message: *mut crate::System::String,
        status: crate::System::Net::WebExceptionStatus,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, status, internalStatus, innerException))?;
        Ok(__cordl_object)
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext9(
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
    pub fn _ctor_String1(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception2(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception_WebExceptionStatus_WebResponse5(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException, status, response))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception_WebExceptionStatus_WebResponse_WebExceptionInternalStatus7(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (message, innerException, status, response, internalStatus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_Exception_WebExceptionStatus_WebResponse6(
        &mut self,
        message: *mut crate::System::String,
        data: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, data, innerException, status, response))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_Exception_WebExceptionStatus_WebResponse_WebExceptionInternalStatus8(
        &mut self,
        message: *mut crate::System::String,
        data: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        status: crate::System::Net::WebExceptionStatus,
        response: *mut crate::System::Net::WebResponse,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (message, data, innerException, status, response, internalStatus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_WebExceptionStatus3(
        &mut self,
        message: *mut crate::System::String,
        status: crate::System::Net::WebExceptionStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, status))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_WebExceptionStatus_WebExceptionInternalStatus_Exception4(
        &mut self,
        message: *mut crate::System::String,
        status: crate::System::Net::WebExceptionStatus,
        internalStatus: crate::System::Net::WebExceptionInternalStatus,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, status, internalStatus, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn get_Response(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebResponse = __cordl_object
            .invoke("get_Response", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::WebExceptionStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::WebExceptionStatus = __cordl_object
            .invoke("get_Status", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebException")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
