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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(bool), bool, 1usize>("CustomErrorsEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CustomErrorsEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (isLocalRequest)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsActivationAllowed(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsActivationAllowed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsActivationAllowed", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (svrType)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsRemotelyActivatedClientType(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
                >,
                1usize,
            >("IsRemotelyActivatedClientType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsRemotelyActivatedClientType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        > = unsafe { method.invoke_unchecked((), (svrType)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsWellKnownClientType(
        svrType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
                >,
                1usize,
            >("IsWellKnownClientType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsWellKnownClientType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        > = unsafe { method.invoke_unchecked((), (svrType)) };
        Ok(__cordl_ret.into())
    }
    pub fn LoadDefaultDelayedChannels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("LoadDefaultDelayedChannels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadDefaultDelayedChannels", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterActivatedClientType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterActivatedClientType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterActivatedClientType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (entry))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterActivatedServiceType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedServiceTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ActivatedServiceTypeEntry,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterActivatedServiceType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterActivatedServiceType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (entry))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannelTemplate(
        channel: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ChannelData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ChannelData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterChannelTemplate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterChannelTemplate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (channel))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannels(
        channels: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        onlyDelayed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterChannels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterChannels", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (channels, onlyDelayed))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterClientProviderTemplate(
        prov: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ProviderData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterClientProviderTemplate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterClientProviderTemplate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (prov))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterServerProviderTemplate(
        prov: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ProviderData,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterServerProviderTemplate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterServerProviderTemplate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (prov))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTypes(
        types: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterTypes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (types))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWellKnownClientType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterWellKnownClientType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterWellKnownClientType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (entry))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWellKnownServiceType(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownServiceTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::WellKnownServiceTypeEntry,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterWellKnownServiceType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterWellKnownServiceType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (entry))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCustomErrorsMode(
        mode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetCustomErrorsMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetCustomErrorsMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ApplicationName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_ApplicationName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ApplicationName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_ProcessId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ProcessId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_ApplicationName(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ApplicationName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ApplicationName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
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
