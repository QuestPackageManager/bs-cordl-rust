#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpRequestHeaders {
    __cordl_parent: crate::System::Net::Http::Headers::HttpHeaders,
    pub expectContinue: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HttpRequestHeaders
    => "System.Net.Http.Headers"."HttpRequestHeaders"
);
#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HttpRequestHeaders {
    type Target = crate::System::Net::Http::Headers::HttpHeaders;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HttpRequestHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
impl crate::System::Net::Http::Headers::HttpRequestHeaders {
    #[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders+__c")]
    pub type __c = crate::System::Net::Http::Headers::HttpRequestHeaders___c;
    pub fn get_TransferEncodingChunked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_TransferEncodingChunked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConnectionClose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_ConnectionClose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Host(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Host", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Authorization(
        &mut self,
        value: *mut crate::System::Net::Http::Headers::AuthenticationHeaderValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Authorization", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddHeaders(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpRequestHeaders,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddHeaders", (headers))?;
        Ok(__cordl_ret)
    }
    pub fn get_Connection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Connection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TransferEncoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<
            *mut crate::System::Net::Http::Headers::TransferCodingHeaderValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<
            *mut crate::System::Net::Http::Headers::TransferCodingHeaderValue,
        > = __cordl_object.invoke("get_TransferEncoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExpectContinue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_ExpectContinue", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpRequestHeaders")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HttpRequestHeaders {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
