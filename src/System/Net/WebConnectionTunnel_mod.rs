#[cfg(feature = "System+Net+WebConnectionTunnel")]
#[repr(C)]
#[derive(Debug)]
pub struct WebConnectionTunnel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Request_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::HttpWebRequest,
    >,
    pub _ConnectUri_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub connectRequest: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    pub ntlmAuthState: crate::System::Net::WebConnectionTunnel_NtlmAuthState,
    pub _Success_k__BackingField: bool,
    pub _CloseConnection_k__BackingField: bool,
    pub _StatusCode_k__BackingField: i32,
    pub _StatusDescription_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _Challenge_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _Headers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::WebHeaderCollection,
    >,
    pub _ProxyVersion_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Version>,
    pub _Data_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
}
#[cfg(feature = "System+Net+WebConnectionTunnel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebConnectionTunnel => "System.Net"
    ."WebConnectionTunnel"
);
#[cfg(feature = "System+Net+WebConnectionTunnel")]
impl std::ops::Deref for crate::System::Net::WebConnectionTunnel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebConnectionTunnel")]
impl std::ops::DerefMut for crate::System::Net::WebConnectionTunnel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebConnectionTunnel")]
impl crate::System::Net::WebConnectionTunnel {
    #[cfg(feature = "System+Net+WebConnectionTunnel+NtlmAuthState")]
    pub type NtlmAuthState = crate::System::Net::WebConnectionTunnel_NtlmAuthState;
    pub fn FlushContents(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        contentLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushContents", (stream, contentLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("Initialize", (stream, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        connectUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, connectUri))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadHeaders(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_3<
                    *mut crate::System::Net::WebHeaderCollection,
                    *mut quest_hook::libil2cpp::Il2CppArray<u8>,
                    i32,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_3<
                    *mut crate::System::Net::WebHeaderCollection,
                    *mut quest_hook::libil2cpp::Il2CppArray<u8>,
                    i32,
                >,
            >,
        > = __cordl_object.invoke("ReadHeaders", (stream, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        connectUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, connectUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Challenge(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_Challenge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CloseConnection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CloseConnection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_ConnectUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Data", ())?;
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
    pub fn get_ProxyVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = __cordl_object
            .invoke("get_ProxyVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Request(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest> = __cordl_object
            .invoke("get_Request", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StatusCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_StatusCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Success(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Success", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Challenge(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Challenge", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CloseConnection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CloseConnection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Data(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Data", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Headers(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::WebHeaderCollection>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Headers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProxyVersion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProxyVersion", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_StatusDescription(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StatusDescription", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Success(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Success", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebConnectionTunnel")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebConnectionTunnel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebConnectionTunnel+NtlmAuthState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebConnectionTunnel_NtlmAuthState {
    #[default]
    Challenge = 1i32,
    None = 0i32,
    Response = 2i32,
}
#[cfg(feature = "System+Net+WebConnectionTunnel+NtlmAuthState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebConnectionTunnel_NtlmAuthState
    => "System.Net"."WebConnectionTunnel/NtlmAuthState"
);
