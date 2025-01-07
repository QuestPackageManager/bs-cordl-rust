#[cfg(feature = "UnityEngine+Time")]
#[repr(C)]
#[derive(Debug)]
pub struct Time {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Time")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Time {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Time";
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
#[cfg(feature = "UnityEngine+Time")]
impl std::ops::Deref for crate::UnityEngine::Time {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Time")]
impl std::ops::DerefMut for crate::UnityEngine::Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Time")]
impl crate::UnityEngine::Time {
    pub fn get_captureDeltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_captureDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_captureFramerate() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_captureFramerate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedDeltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedUnscaledTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedUnscaledTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_frameCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_frameCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumDeltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maximumDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_realtimeSinceStartup() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_realtimeSinceStartup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_smoothDeltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_smoothDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_timeScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeSinceLevelLoad() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_timeSinceLevelLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unscaledDeltaTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unscaledDeltaTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unscaledTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unscaledTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_captureDeltaTime(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_captureDeltaTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_captureFramerate(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_captureFramerate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_timeScale(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_timeScale", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Time")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Time {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
