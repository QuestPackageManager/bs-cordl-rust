#[cfg(feature = "OVR+OpenVR+CVRApplications")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRApplications {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRApplications,
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRApplications {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRApplications";
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
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRApplications {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRApplications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl crate::OVR::OpenVR::CVRApplications {
    pub fn AddApplicationManifest(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bTemporary: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, bool),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("AddApplicationManifest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddApplicationManifest", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchApplicationManifestFullPath, bTemporary))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CancelApplicationLaunch(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("CancelApplicationLaunch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CancelApplicationLaunch", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pchAppKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationAutoLaunch(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("GetApplicationAutoLaunch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationAutoLaunch", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pchAppKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("GetApplicationCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationCount", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationKeyByIndex(
        &mut self,
        unApplicationIndex: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                3usize,
            >("GetApplicationKeyByIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationKeyByIndex", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unApplicationIndex, pchAppKeyBuffer, unAppKeyBufferLen),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationKeyByProcessId(
        &mut self,
        unProcessId: u32,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                3usize,
            >("GetApplicationKeyByProcessId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationKeyByProcessId", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (unProcessId, pchAppKeyBuffer, unAppKeyBufferLen),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationLaunchArguments(
        &mut self,
        unHandle: u32,
        pchArgs: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unArgs: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                u32,
                3usize,
            >("GetApplicationLaunchArguments")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationLaunchArguments", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked(self, (unHandle, pchArgs, unArgs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationProcessId(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                u32,
                1usize,
            >("GetApplicationProcessId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationProcessId", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (pchAppKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationPropertyBool(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::OVR::OpenVR::EVRApplicationProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRApplicationError,
                    >,
                ),
                bool,
                3usize,
            >("GetApplicationPropertyBool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationPropertyBool", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pchAppKey, eProperty, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationPropertyString(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        pchPropertyValueBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unPropertyValueBufferLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::OVR::OpenVR::EVRApplicationProperty,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRApplicationError,
                    >,
                ),
                u32,
                5usize,
            >("GetApplicationPropertyString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationPropertyString", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pchAppKey,
                        eProperty,
                        pchPropertyValueBuffer,
                        unPropertyValueBufferLen,
                        peError,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationPropertyUint64(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eProperty: crate::OVR::OpenVR::EVRApplicationProperty,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRApplicationError>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::OVR::OpenVR::EVRApplicationProperty,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::OVR::OpenVR::EVRApplicationError,
                    >,
                ),
                u64,
                3usize,
            >("GetApplicationPropertyUint64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationPropertyUint64", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked(self, (pchAppKey, eProperty, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationSupportedMimeTypes(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeTypesBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unMimeTypesBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                bool,
                3usize,
            >("GetApplicationSupportedMimeTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationSupportedMimeTypes", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pchAppKey, pchMimeTypesBuffer, unMimeTypesBuffer),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationsErrorNameFromEnum(
        &mut self,
        error: crate::OVR::OpenVR::EVRApplicationError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVRApplicationError),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetApplicationsErrorNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationsErrorNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (error)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationsThatSupportMimeType(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeysThatSupportBuffer: quest_hook::libil2cpp::Gc<
            crate::System::Text::StringBuilder,
        >,
        unAppKeysThatSupportBuffer: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                u32,
                3usize,
            >("GetApplicationsThatSupportMimeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationsThatSupportMimeType", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        pchMimeType,
                        pchAppKeysThatSupportBuffer,
                        unAppKeysThatSupportBuffer,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationsTransitionStateNameFromEnum(
        &mut self,
        state: crate::OVR::OpenVR::EVRApplicationTransitionState,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVRApplicationTransitionState),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetApplicationsTransitionStateNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetApplicationsTransitionStateNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (state)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentSceneProcessId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("GetCurrentSceneProcessId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCurrentSceneProcessId", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultApplicationForMimeType(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                ),
                bool,
                3usize,
            >("GetDefaultApplicationForMimeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDefaultApplicationForMimeType", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pchMimeType, pchAppKeyBuffer, unAppKeyBufferLen),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStartingApplication(
        &mut self,
        pchAppKeyBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unAppKeyBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>, u32),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("GetStartingApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStartingApplication", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKeyBuffer, unAppKeyBufferLen))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransitionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::OVR::OpenVR::EVRApplicationTransitionState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::OVR::OpenVR::EVRApplicationTransitionState,
                0usize,
            >("GetTransitionState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTransitionState", 0usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationTransitionState = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn IdentifyApplication(
        &mut self,
        unProcessId: u32,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("IdentifyApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IdentifyApplication", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (unProcessId, pchAppKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsApplicationInstalled(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsApplicationInstalled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsApplicationInstalled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pchAppKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsQuitUserPromptRequested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsQuitUserPromptRequested")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsQuitUserPromptRequested", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchApplication(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::OVR::OpenVR::EVRApplicationError,
                1usize,
            >("LaunchApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchApplication", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchApplicationFromMimeType(
        &mut self,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArgs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("LaunchApplicationFromMimeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchApplicationFromMimeType", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchMimeType, pchArgs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchDashboardOverlay(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::OVR::OpenVR::EVRApplicationError,
                1usize,
            >("LaunchDashboardOverlay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchDashboardOverlay", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchInternalProcess(
        &mut self,
        pchBinaryPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchArguments: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchWorkingDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                3usize,
            >("LaunchInternalProcess")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchInternalProcess", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pchBinaryPath, pchArguments, pchWorkingDirectory),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchTemplateApplication(
        &mut self,
        pchTemplateAppKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchNewAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pKeys: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::OVR::OpenVR::AppOverrideKeys_t>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::OVR::OpenVR::AppOverrideKeys_t,
                        >,
                    >,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                3usize,
            >("LaunchTemplateApplication")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchTemplateApplication", 3usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchTemplateAppKey, pchNewAppKey, pKeys))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn PerformApplicationPrelaunchCheck(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::OVR::OpenVR::EVRApplicationError,
                1usize,
            >("PerformApplicationPrelaunchCheck")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PerformApplicationPrelaunchCheck", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveApplicationManifest(
        &mut self,
        pchApplicationManifestFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::OVR::OpenVR::EVRApplicationError,
                1usize,
            >("RemoveApplicationManifest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveApplicationManifest", 1usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchApplicationManifestFullPath))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetApplicationAutoLaunch(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bAutoLaunch: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, bool),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("SetApplicationAutoLaunch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetApplicationAutoLaunch", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKey, bAutoLaunch))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultApplicationForMimeType(
        &mut self,
        pchAppKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchMimeType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRApplicationError> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                crate::OVR::OpenVR::EVRApplicationError,
                2usize,
            >("SetDefaultApplicationForMimeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDefaultApplicationForMimeType", 2usize
                )
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRApplicationError = unsafe {
            method.invoke_unchecked(self, (pchAppKey, pchMimeType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRApplications")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRApplications {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
