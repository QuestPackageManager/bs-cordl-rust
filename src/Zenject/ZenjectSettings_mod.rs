#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectSettings_SignalSettings {
    __cordl_parent: crate::System::Object,
    pub _defaultSyncMode: crate::Zenject::SignalDefaultSyncModes,
    pub _missingHandlerDefaultResponse: crate::Zenject::SignalMissingHandlerResponses,
    pub _requireStrictUnsubscribe: bool,
    pub _defaultAsyncTickPriority: i32,
}
#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectSettings_SignalSettings =>
    "Zenject"."ZenjectSettings/SignalSettings"
);
#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
impl std::ops::Deref for crate::Zenject::ZenjectSettings_SignalSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
impl std::ops::DerefMut for crate::Zenject::ZenjectSettings_SignalSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
impl crate::Zenject::ZenjectSettings_SignalSettings {
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SignalDefaultSyncModes_SignalMissingHandlerResponses__cordl_bool_i32_0(
        defaultSyncMode: crate::Zenject::SignalDefaultSyncModes,
        missingHandlerDefaultResponse: crate::Zenject::SignalMissingHandlerResponses,
        requireStrictUnsubscribe: bool,
        defaultAsyncTickPriority: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    defaultSyncMode,
                    missingHandlerDefaultResponse,
                    requireStrictUnsubscribe,
                    defaultAsyncTickPriority,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignalDefaultSyncModes_SignalMissingHandlerResponses__cordl_bool_i32_0(
        &mut self,
        defaultSyncMode: crate::Zenject::SignalDefaultSyncModes,
        missingHandlerDefaultResponse: crate::Zenject::SignalMissingHandlerResponses,
        requireStrictUnsubscribe: bool,
        defaultAsyncTickPriority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    defaultSyncMode,
                    missingHandlerDefaultResponse,
                    requireStrictUnsubscribe,
                    defaultAsyncTickPriority,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultAsyncTickPriority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DefaultAsyncTickPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultSyncMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::SignalDefaultSyncModes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::SignalDefaultSyncModes = __cordl_object
            .invoke("get_DefaultSyncMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MissingHandlerDefaultResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::SignalMissingHandlerResponses> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::SignalMissingHandlerResponses = __cordl_object
            .invoke("get_MissingHandlerDefaultResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequireStrictUnsubscribe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequireStrictUnsubscribe", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ZenjectSettings_SignalSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ZenjectSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectSettings {
    __cordl_parent: crate::System::Object,
    pub _ensureDeterministicDestructionOrderOnApplicationQuit: bool,
    pub _displayWarningWhenResolvingDuringInstall: bool,
    pub _validationRootResolveMethod: crate::Zenject::RootResolveMethods,
    pub _validationErrorResponse: crate::Zenject::ValidationErrorResponses,
    pub _signalSettings: *mut crate::Zenject::ZenjectSettings_SignalSettings,
}
#[cfg(feature = "Zenject+ZenjectSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectSettings => "Zenject"
    ."ZenjectSettings"
);
#[cfg(feature = "Zenject+ZenjectSettings")]
impl std::ops::Deref for crate::Zenject::ZenjectSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSettings")]
impl std::ops::DerefMut for crate::Zenject::ZenjectSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectSettings")]
impl crate::Zenject::ZenjectSettings {
    #[cfg(feature = "Zenject+ZenjectSettings+SignalSettings")]
    pub type SignalSettings = crate::Zenject::ZenjectSettings_SignalSettings;
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ValidationErrorResponses_RootResolveMethods__cordl_bool__cordl_bool_ZenjectSettings_SignalSettings0(
        validationErrorResponse: crate::Zenject::ValidationErrorResponses,
        validationRootResolveMethod: crate::Zenject::RootResolveMethods,
        displayWarningWhenResolvingDuringInstall: bool,
        ensureDeterministicDestructionOrderOnApplicationQuit: bool,
        signalSettings: *mut crate::Zenject::ZenjectSettings_SignalSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    validationErrorResponse,
                    validationRootResolveMethod,
                    displayWarningWhenResolvingDuringInstall,
                    ensureDeterministicDestructionOrderOnApplicationQuit,
                    signalSettings,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ValidationErrorResponses_RootResolveMethods__cordl_bool__cordl_bool_ZenjectSettings_SignalSettings0(
        &mut self,
        validationErrorResponse: crate::Zenject::ValidationErrorResponses,
        validationRootResolveMethod: crate::Zenject::RootResolveMethods,
        displayWarningWhenResolvingDuringInstall: bool,
        ensureDeterministicDestructionOrderOnApplicationQuit: bool,
        signalSettings: *mut crate::Zenject::ZenjectSettings_SignalSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    validationErrorResponse,
                    validationRootResolveMethod,
                    displayWarningWhenResolvingDuringInstall,
                    ensureDeterministicDestructionOrderOnApplicationQuit,
                    signalSettings,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayWarningWhenResolvingDuringInstall(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DisplayWarningWhenResolvingDuringInstall", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnsureDeterministicDestructionOrderOnApplicationQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EnsureDeterministicDestructionOrderOnApplicationQuit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Signals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ZenjectSettings_SignalSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ZenjectSettings_SignalSettings = __cordl_object
            .invoke("get_Signals", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidationErrorResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::ValidationErrorResponses> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::ValidationErrorResponses = __cordl_object
            .invoke("get_ValidationErrorResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidationRootResolveMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::RootResolveMethods> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::RootResolveMethods = __cordl_object
            .invoke("get_ValidationRootResolveMethod", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+ZenjectSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
