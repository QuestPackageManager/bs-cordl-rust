#[cfg(feature = "UnityEngine+AudioSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AudioSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AudioSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioSettings";
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
#[cfg(feature = "UnityEngine+AudioSettings")]
impl std::ops::Deref for crate::UnityEngine::AudioSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl std::ops::DerefMut for crate::UnityEngine::AudioSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl crate::UnityEngine::AudioSettings {
    #[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
    pub type AudioConfigurationChangeHandler = crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler;
    #[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
    pub type Mobile = crate::UnityEngine::AudioSettings_Mobile;
    pub fn GetConfiguration() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::AudioConfiguration,
    > {
        let __cordl_ret: crate::UnityEngine::AudioConfiguration = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConfiguration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConfiguration_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::AudioConfiguration>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConfiguration_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSampleRate() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSampleRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpatializerPluginName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpatializerPluginName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnAudioConfigurationChanged(
        deviceWasChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnAudioConfigurationChanged", (deviceWasChanged))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnAudioSystemShuttingDown() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnAudioSystemShuttingDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnAudioSystemStartedUp() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnAudioSystemStartedUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAudioOutput() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartAudioOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopAudioOutput() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopAudioOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_OnAudioConfigurationChanged(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_OnAudioConfigurationChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dspTime() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_dspTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_outputSampleRate() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_outputSampleRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnAudioConfigurationChanged(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_OnAudioConfigurationChanged", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings_AudioConfigurationChangeHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioConfigurationChangeHandler";
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
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl std::ops::Deref
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl std::ops::DerefMut
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    pub fn Invoke(
        &mut self,
        deviceWasChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (deviceWasChanged))?;
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
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings_Mobile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AudioSettings_Mobile {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Mobile";
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
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl std::ops::Deref for crate::UnityEngine::AudioSettings_Mobile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl std::ops::DerefMut for crate::UnityEngine::AudioSettings_Mobile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl crate::UnityEngine::AudioSettings_Mobile {
    pub fn InvokeIsStopAudioOutputOnMuteEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeIsStopAudioOutputOnMuteEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnMuteStateChanged(
        mute: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeOnMuteStateChanged", (mute))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAudioOutput() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartAudioOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopAudioOutput() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopAudioOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_muteState() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_muteState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stopAudioOutputOnMute() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_stopAudioOutputOnMute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_muteState(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_muteState", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioSettings_Mobile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
