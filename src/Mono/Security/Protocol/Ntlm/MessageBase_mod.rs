#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageBase {
    __cordl_parent: crate::System::Object,
    pub _type: i32,
    pub _flags: crate::Mono::Security::Protocol::Ntlm::NtlmFlags,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Protocol::Ntlm::MessageBase =>
    "Mono.Security.Protocol.Ntlm"."MessageBase"
);
#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
impl std::ops::Deref for crate::Mono::Security::Protocol::Ntlm::MessageBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
impl std::ops::DerefMut for crate::Mono::Security::Protocol::Ntlm::MessageBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
impl crate::Mono::Security::Protocol::Ntlm::MessageBase {
    pub fn CheckHeader(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckHeader", (message))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", (message))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(messageType: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (messageType))?;
        Ok(__cordl_object)
    }
    pub fn PrepareMessage(
        &mut self,
        messageSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("PrepareMessage", (messageSize))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        messageType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (messageType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Security::Protocol::Ntlm::NtlmFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Security::Protocol::Ntlm::NtlmFlags = __cordl_object
            .invoke("get_Flags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Flags(
        &mut self,
        value: crate::Mono::Security::Protocol::Ntlm::NtlmFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Flags", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+MessageBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Protocol::Ntlm::MessageBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
