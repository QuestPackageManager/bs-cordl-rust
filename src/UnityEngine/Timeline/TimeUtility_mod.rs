#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::TimeUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimeUtility";
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
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl crate::UnityEngine::Timeline::TimeUtility {
    pub fn FromFrames_f64_1(
        frames: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFrames", (frames, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFrames_i32_0(
        frames: i32,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFrames", (frames, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAnimationClipLength(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAnimationClipLength", (clip))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClosestFrameRate(
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::FrameRate> {
        let __cordl_ret: crate::UnityEngine::Playables::FrameRate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClosestFrameRate", (frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEpsilon(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEpsilon", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFrame(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NextFrame", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFrameTime(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NextFrameTime", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFrameBoundary_f64_1(
        _cordl_time: f64,
        frameRate: f64,
        epsilon: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnFrameBoundary", (_cordl_time, frameRate, epsilon))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFrameBoundary_f64_f64_0(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnFrameBoundary", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeCode(
        timeCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        frameRate: f64,
        defaultValue: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTimeCode", (timeCode, frameRate, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeSeconds(
        timeCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        frameRate: f64,
        defaultValue: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTimeSeconds", (timeCode, frameRate, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreviousFrame(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreviousFrame", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreviousFrameTime(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreviousFrameTime", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChar(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charToRemoveFunc: quest_hook::libil2cpp::Gc<crate::System::Func_2<char, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveChar", (str, charToRemoveFunc))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundToFrame(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundToFrame", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn TimeAsFrames(
        timeValue: f64,
        frameRate: f64,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TimeAsFrames", (timeValue, frameRate, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn TimeAsTimeCode(
        timeValue: f64,
        frameRate: f64,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TimeAsTimeCode", (timeValue, frameRate, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToExactFrames(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToExactFrames", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFrameRate(
        enumValue: crate::UnityEngine::Timeline::StandardFrameRates,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::FrameRate> {
        let __cordl_ret: crate::UnityEngine::Playables::FrameRate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFrameRate", (enumValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFrames(
        _cordl_time: f64,
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFrames", (_cordl_time, frameRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToStandardFrameRate(
        rate: crate::UnityEngine::Playables::FrameRate,
        standard: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Timeline::StandardFrameRates,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToStandardFrameRate", (rate, standard))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFrameRate(
        frameRate: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateFrameRate", (frameRate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::TimeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
