#[cfg(feature = "System+Net+WebProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct WebProxy {
    __cordl_parent: crate::System::Object,
    pub _UseRegistry: bool,
    pub _BypassOnLocal: bool,
    pub m_EnableAutoproxy: bool,
    pub _ProxyAddress: *mut crate::System::Uri,
    pub _BypassList: *mut crate::System::Collections::ArrayList,
    pub _Credentials: *mut crate::System::Net::ICredentials,
    pub _RegExBypassList: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Text::RegularExpressions::Regex,
    >,
    pub _ProxyHostAddresses: *mut crate::System::Collections::Hashtable,
    pub m_ScriptEngine: *mut crate::System::Net::AutoWebProxyScriptEngine,
}
#[cfg(feature = "System+Net+WebProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebProxy => "System.Net"."WebProxy"
);
#[cfg(feature = "System+Net+WebProxy")]
impl std::ops::Deref for crate::System::Net::WebProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl std::ops::DerefMut for crate::System::Net::WebProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl crate::System::Net::WebProxy {
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
    pub fn GetProxy(
        &mut self,
        destination: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("GetProxy", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn GetProxyAuto(
        &mut self,
        destination: *mut crate::System::Uri,
        proxyUri: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetProxyAuto", (destination, proxyUri))?;
        Ok(__cordl_ret)
    }
    pub fn IsBypassed(
        &mut self,
        host: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBypassed", (host))?;
        Ok(__cordl_ret)
    }
    pub fn IsBypassedAuto(
        &mut self,
        destination: *mut crate::System::Uri,
        isBypassed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBypassedAuto", (destination, isBypassed))?;
        Ok(__cordl_ret)
    }
    pub fn IsBypassedManual(
        &mut self,
        host: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBypassedManual", (host))?;
        Ok(__cordl_ret)
    }
    pub fn IsLocal(
        &mut self,
        host: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLocal", (host))?;
        Ok(__cordl_ret)
    }
    pub fn IsLocalInProxyHash(
        &mut self,
        host: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLocalInProxyHash", (host))?;
        Ok(__cordl_ret)
    }
    pub fn IsMatchInBypassList(
        &mut self,
        input: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatchInBypassList", (input))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn New_Uri__cordl_bool_Il2CppArray_ICredentials1(
        Address: *mut crate::System::Uri,
        BypassOnLocal: bool,
        BypassList: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        Credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (Address, BypassOnLocal, BypassList, Credentials))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool3(
        enableAutoproxy: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enableAutoproxy))?;
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
    pub fn UnsafeUpdateFromRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsafeUpdateFromRegistry", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateRegExList(
        &mut self,
        canThrow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRegExList", (canThrow))?;
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
    pub fn _ctor_SerializationInfo_StreamingContext2(
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
    pub fn _ctor_Uri__cordl_bool_Il2CppArray_ICredentials1(
        &mut self,
        Address: *mut crate::System::Uri,
        BypassOnLocal: bool,
        BypassList: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        Credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (Address, BypassOnLocal, BypassList, Credentials))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        enableAutoproxy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (enableAutoproxy))?;
        Ok(__cordl_ret)
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ICredentials> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ICredentials = __cordl_object
            .invoke("get_Credentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ScriptEngine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::AutoWebProxyScriptEngine,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::AutoWebProxyScriptEngine = __cordl_object
            .invoke("get_ScriptEngine", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseDefaultCredentials(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseDefaultCredentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UseDefaultCredentials(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseDefaultCredentials", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
