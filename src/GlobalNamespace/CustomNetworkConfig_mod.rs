#[cfg(feature = "CustomNetworkConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNetworkConfig {
    __cordl_parent: crate::System::Object,
    pub _maxPartySize_k__BackingField: i32,
    pub _discoveryPort_k__BackingField: i32,
    pub _partyPort_k__BackingField: i32,
    pub _multiplayerPort_k__BackingField: i32,
    pub _masterServerEndPoint_k__BackingField: *mut crate::GlobalNamespace::DnsEndPoint,
    pub _multiplayerStatusUrl_k__BackingField: *mut crate::System::String,
    pub _graphUrl_k__BackingField: *mut crate::System::String,
    pub _graphAccessToken_k__BackingField: *mut crate::System::String,
    pub _forceGameLift_k__BackingField: bool,
    pub _serviceEnvironment_k__BackingField: crate::GlobalNamespace::ServiceEnvironment,
}
#[cfg(feature = "CustomNetworkConfig")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CustomNetworkConfig => ""
    ."CustomNetworkConfig"
);
#[cfg(feature = "CustomNetworkConfig")]
impl std::ops::Deref for crate::GlobalNamespace::CustomNetworkConfig {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CustomNetworkConfig")]
impl std::ops::DerefMut for crate::GlobalNamespace::CustomNetworkConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CustomNetworkConfig")]
impl crate::GlobalNamespace::CustomNetworkConfig {
    pub fn New(
        fromNetworkConfig: *mut crate::GlobalNamespace::INetworkConfig,
        customServerHostName: *mut crate::System::String,
        port: i32,
        forceGameLift: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fromNetworkConfig, customServerHostName, port, forceGameLift),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        fromNetworkConfig: *mut crate::GlobalNamespace::INetworkConfig,
        customServerHostName: *mut crate::System::String,
        port: i32,
        forceGameLift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (fromNetworkConfig, customServerHostName, port, forceGameLift),
            )?;
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
    pub fn get_discoveryPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_discoveryPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_forceGameLift(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_forceGameLift", ())?;
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
    pub fn get_masterServerEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::DnsEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::DnsEndPoint = __cordl_object
            .invoke("get_masterServerEndPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxPartySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxPartySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multiplayerPort", ())?;
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
    pub fn get_partyPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_partyPort", ())?;
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
    pub fn get_serviceEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ServiceEnvironment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ServiceEnvironment = __cordl_object
            .invoke("get_serviceEnvironment", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CustomNetworkConfig")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CustomNetworkConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
