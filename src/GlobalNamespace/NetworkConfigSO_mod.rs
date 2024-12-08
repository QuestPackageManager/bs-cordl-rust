#[cfg(feature = "NetworkConfigSO")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkConfigSO {
    __cordl_parent: PersistentScriptableObject,
    pub _maxPartySize: i32,
    pub _discoveryPort: i32,
    pub _partyPort: i32,
    pub _multiplayerPort: i32,
    pub _masterServerPort: i32,
    pub _masterServerHostName: *mut crate::System::String,
    pub _multiplayerStatusUrl: *mut crate::System::String,
    pub _quickPlaySetupUrl: *mut crate::System::String,
    pub _graphUrl: *mut crate::System::String,
    pub _graphAppId: *mut crate::System::String,
    pub _forceGameLift: bool,
    pub _serviceEnvironment: ServiceEnvironment,
}
#[cfg(feature = "NetworkConfigSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NetworkConfigSO => ""."NetworkConfigSO"
);
#[cfg(feature = "NetworkConfigSO")]
impl std::ops::Deref for NetworkConfigSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConfigSO")]
impl std::ops::DerefMut for NetworkConfigSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConfigSO")]
impl NetworkConfigSO {
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
    pub fn get_masterServerEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut DnsEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut DnsEndPoint = __cordl_object
            .invoke("get_masterServerEndPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_quickPlaySetupUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_quickPlaySetupUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_discoveryPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_discoveryPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_graphUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_graphUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_graphAccessToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_graphAccessToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxPartySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxPartySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_forceGameLift(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_forceGameLift", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerStatusUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_multiplayerStatusUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serviceEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<ServiceEnvironment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ServiceEnvironment = __cordl_object
            .invoke("get_serviceEnvironment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multiplayerPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_partyPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_partyPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_appId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_appId", ())?;
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
#[cfg(feature = "NetworkConfigSO")]
impl quest_hook::libil2cpp::ObjectType for NetworkConfigSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
