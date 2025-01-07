#[cfg(feature = "UnityEngine+Application")]
#[repr(C)]
#[derive(Debug)]
pub struct Application {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Application")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Application {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Application";
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
#[cfg(feature = "UnityEngine+Application")]
impl std::ops::Deref for crate::UnityEngine::Application {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application")]
impl std::ops::DerefMut for crate::UnityEngine::Application {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application")]
impl crate::UnityEngine::Application {
    #[cfg(feature = "UnityEngine+Application+LogCallback")]
    pub type LogCallback = crate::UnityEngine::Application_LogCallback;
    #[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
    pub type LowMemoryCallback = crate::UnityEngine::Application_LowMemoryCallback;
    #[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
    pub type MemoryUsageChangedCallback = crate::UnityEngine::Application_MemoryUsageChangedCallback;
    pub fn CallLogCallback(
        logString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        stackTrace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::LogType,
        invokedOnMainThread: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CallLogCallback",
                (logString, stackTrace, _cordl_type, invokedOnMainThread),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CallLowMemory(
        usage: crate::UnityEngine::ApplicationMemoryUsage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CallLowMemory", (usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanStreamedLevelBeLoaded_Il2CppString1(
        levelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanStreamedLevelBeLoaded", (levelName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanStreamedLevelBeLoaded_i32_0(
        levelIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanStreamedLevelBeLoaded", (levelIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLogCallback() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasLogCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ApplicationInit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ApplicationInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ApplicationQuit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ApplicationQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ApplicationUnload() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ApplicationUnload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ApplicationWantsToQuit() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ApplicationWantsToQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeDeepLinkActivated(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeDeepLinkActivated", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeFocusChanged(
        focus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeFocusChanged", (focus))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnBeforeRender() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnBeforeRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenURL(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenURL", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn Quit_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Quit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Quit_i32_0(
        exitCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Quit", (exitCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLogCallbackDefined(
        defined: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLogCallbackDefined", (defined))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_focusChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_focusChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_logMessageReceived(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Application_LogCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_logMessageReceived", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onBeforeRender(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onBeforeRender", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_quitting(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_quitting", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundLoadingPriority() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ThreadPriority,
    > {
        let __cordl_ret: crate::UnityEngine::ThreadPriority = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundLoadingPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buildGUID() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_buildGUID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_companyName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_companyName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dataPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_dataPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identifier() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_identifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_installMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ApplicationInstallMode,
    > {
        let __cordl_ret: crate::UnityEngine::ApplicationInstallMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_installMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_installerName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_installerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_internetReachability() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::NetworkReachability,
    > {
        let __cordl_ret: crate::UnityEngine::NetworkReachability = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_internetReachability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isBatchMode() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isBatchMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConsolePlatform() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isConsolePlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEditor() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isEditor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFocused() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isFocused", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isMobilePlatform() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isMobilePlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPlaying() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isPlaying", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_persistentDataPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_persistentDataPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platform() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::RuntimePlatform,
    > {
        let __cordl_ret: crate::UnityEngine::RuntimePlatform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_platform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_productName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_productName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_runInBackground() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_runInBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sandboxType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ApplicationSandboxType,
    > {
        let __cordl_ret: crate::UnityEngine::ApplicationSandboxType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sandboxType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_streamingAssetsPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_streamingAssetsPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemLanguage() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SystemLanguage,
    > {
        let __cordl_ret: crate::UnityEngine::SystemLanguage = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemLanguage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetFrameRate() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_targetFrameRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_temporaryCachePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_temporaryCachePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_focusChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_focusChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_logMessageReceived(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Application_LogCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_logMessageReceived", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onBeforeRender(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onBeforeRender", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_quitting(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_quitting", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundLoadingPriority(
        value: crate::UnityEngine::ThreadPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_backgroundLoadingPriority", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_runInBackground(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_runInBackground", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetFrameRate(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_targetFrameRate", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Application")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Application {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Application+LogCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Application_LogCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Application+LogCallback")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Application_LogCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "LogCallback";
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
#[cfg(feature = "UnityEngine+Application+LogCallback")]
impl std::ops::Deref for crate::UnityEngine::Application_LogCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+LogCallback")]
impl std::ops::DerefMut for crate::UnityEngine::Application_LogCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+LogCallback")]
impl crate::UnityEngine::Application_LogCallback {
    pub fn Invoke(
        &mut self,
        condition: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        stackTrace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (condition, stackTrace, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Application+LogCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Application_LogCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Application_LowMemoryCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Application_LowMemoryCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "LowMemoryCallback";
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
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
impl std::ops::Deref for crate::UnityEngine::Application_LowMemoryCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
impl std::ops::DerefMut for crate::UnityEngine::Application_LowMemoryCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
impl crate::UnityEngine::Application_LowMemoryCallback {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Application+LowMemoryCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Application_LowMemoryCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Application_MemoryUsageChangedCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Application_MemoryUsageChangedCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "MemoryUsageChangedCallback";
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
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
impl std::ops::Deref for crate::UnityEngine::Application_MemoryUsageChangedCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
impl std::ops::DerefMut for crate::UnityEngine::Application_MemoryUsageChangedCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
impl crate::UnityEngine::Application_MemoryUsageChangedCallback {
    pub fn Invoke(
        &mut self,
        usage: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ApplicationMemoryUsageChange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Application+MemoryUsageChangedCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Application_MemoryUsageChangedCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
