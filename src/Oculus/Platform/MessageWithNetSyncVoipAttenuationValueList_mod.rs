#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageWithNetSyncVoipAttenuationValueList {
    __cordl_parent: crate::Oculus::Platform::Message_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        >,
    >,
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::MessageWithNetSyncVoipAttenuationValueList => "Oculus.Platform"
    ."MessageWithNetSyncVoipAttenuationValueList"
);
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
impl std::ops::Deref
for crate::Oculus::Platform::MessageWithNetSyncVoipAttenuationValueList {
    type Target = crate::Oculus::Platform::Message_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
impl std::ops::DerefMut
for crate::Oculus::Platform::MessageWithNetSyncVoipAttenuationValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
impl crate::Oculus::Platform::MessageWithNetSyncVoipAttenuationValueList {
    pub fn GetDataFromMessage(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        > = __cordl_object.invoke("GetDataFromMessage", (c_message))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncVoipAttenuationValueList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        > = __cordl_object.invoke("GetNetSyncVoipAttenuationValueList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c_message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+MessageWithNetSyncVoipAttenuationValueList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::MessageWithNetSyncVoipAttenuationValueList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
