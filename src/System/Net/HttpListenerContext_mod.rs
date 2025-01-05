#[cfg(feature = "System+Net+HttpListenerContext")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListenerContext {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerRequest>,
    pub response: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerResponse>,
    pub user: quest_hook::libil2cpp::Gc<crate::System::Security::Principal::IPrincipal>,
    pub cnc: quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
    pub error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub err_status: i32,
    pub Listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
}
#[cfg(feature = "System+Net+HttpListenerContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListenerContext => "System.Net"
    ."HttpListenerContext"
);
#[cfg(feature = "System+Net+HttpListenerContext")]
impl std::ops::Deref for crate::System::Net::HttpListenerContext {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerContext")]
impl std::ops::DerefMut for crate::System::Net::HttpListenerContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerContext")]
impl crate::System::Net::HttpListenerContext {
    pub fn New(
        cnc: quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cnc))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseAuthentication(
        &mut self,
        expectedSchemes: crate::System::Net::AuthenticationSchemes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAuthentication", (expectedSchemes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseBasicAuthentication(
        &mut self,
        authData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Principal::IPrincipal>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::IPrincipal,
        > = __cordl_object.invoke("ParseBasicAuthentication", (authData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        cnc: quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cnc))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Connection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpConnection> = __cordl_object
            .invoke("get_Connection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ErrorMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ErrorMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ErrorStatus(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ErrorStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HaveError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HaveError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Request(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::HttpListenerRequest,
        > = __cordl_object.invoke("get_Request", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Response(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerResponse>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::HttpListenerResponse,
        > = __cordl_object.invoke("get_Response", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ErrorMessage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ErrorMessage", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ErrorStatus(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ErrorStatus", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpListenerContext")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpListenerContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
