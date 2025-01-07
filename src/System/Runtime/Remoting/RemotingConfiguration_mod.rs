#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingConfiguration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::RemotingConfiguration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "RemotingConfiguration";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingConfiguration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl crate::System::Runtime::Remoting::RemotingConfiguration {
    pub fn CustomErrorsEnabled(
        isLocalRequest: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CustomErrorsEnabled", (isLocalRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActivationAllowed(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsActivationAllowed", (svrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRemotelyActivatedClientType(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRemotelyActivatedClientType", (svrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWellKnownClientType(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWellKnownClientType", (svrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDefaultDelayedChannels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadDefaultDelayedChannels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterActivatedClientType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterActivatedClientType", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterActivatedServiceType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedServiceTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterActivatedServiceType", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannelTemplate(
        channel: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ChannelData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterChannelTemplate", (channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannels(
        channels: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        onlyDelayed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterChannels", (channels, onlyDelayed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterClientProviderTemplate(
        prov: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterClientProviderTemplate", (prov))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterServerProviderTemplate(
        prov: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterServerProviderTemplate", (prov))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTypes(
        types: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterTypes", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWellKnownClientType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterWellKnownClientType", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWellKnownServiceType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownServiceTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterWellKnownServiceType", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCustomErrorsMode(
        mode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCustomErrorsMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ApplicationName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ApplicationName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_ProcessId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ApplicationName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ApplicationName", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingConfiguration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
