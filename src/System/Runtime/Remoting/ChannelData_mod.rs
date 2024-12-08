#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
#[repr(C)]
#[derive(Debug)]
pub struct ChannelData {
    __cordl_parent: crate::System::Object,
    pub Ref: *mut crate::System::String,
    pub Type: *mut crate::System::String,
    pub Id: *mut crate::System::String,
    pub DelayLoadAsClientChannel: *mut crate::System::String,
    pub _serverProviders: *mut crate::System::Collections::ArrayList,
    pub _clientProviders: *mut crate::System::Collections::ArrayList,
    pub _customProperties: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::ChannelData =>
    "System.Runtime.Remoting"."ChannelData"
);
#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ChannelData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ChannelData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
impl crate::System::Runtime::Remoting::ChannelData {
    pub fn CopyFrom(
        &mut self,
        other: *mut crate::System::Runtime::Remoting::ChannelData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (other))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_ClientProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_ClientProviders", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CustomProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("get_CustomProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ServerProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_ServerProviders", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+ChannelData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ChannelData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
