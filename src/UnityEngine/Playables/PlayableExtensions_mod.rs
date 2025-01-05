#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableExtensions =>
    "UnityEngine.Playables"."PlayableExtensions"
);
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl crate::UnityEngine::Playables::PlayableExtensions {
    pub fn GetDuration<U>(playable: U) -> quest_hook::libil2cpp::Result<f64>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDuration", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraph<U>(
        playable: U,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableGraph>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableGraph = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraph", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInput<U>(
        playable: U,
        inputPort: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInput", (playable, inputPort))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputCount<U>(playable: U) -> quest_hook::libil2cpp::Result<i32>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputCount", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputWeight<U>(
        playable: U,
        inputIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputWeight", (playable, inputIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayState<U>(
        playable: U,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayState>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Playables::PlayState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayState", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousTime<U>(playable: U) -> quest_hook::libil2cpp::Result<f64>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviousTime", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTime<U>(playable: U) -> quest_hook::libil2cpp::Result<f64>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTime", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeWrapMode<U>(
        playable: U,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::DirectorWrapMode>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::Playables::DirectorWrapMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeWrapMode", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDone<U>(playable: U) -> quest_hook::libil2cpp::Result<bool>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDone", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid<U>(playable: U) -> quest_hook::libil2cpp::Result<bool>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause<U>(
        playable: U,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pause", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Play<U>(
        playable: U,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Play", (playable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDuration<U>(
        playable: U,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDuration", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputCount<U>(
        playable: U,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputCount", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeight_V1<U, V>(
        playable: U,
        input: V,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        V: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputWeight", (playable, input, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeight_i32_0<U>(
        playable: U,
        inputIndex: i32,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputWeight", (playable, inputIndex, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropagateSetTime<U>(
        playable: U,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPropagateSetTime", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpeed<U>(
        playable: U,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSpeed", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTime<U>(
        playable: U,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTime", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTimeWrapMode<U>(
        playable: U,
        value: crate::UnityEngine::Playables::DirectorWrapMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTimeWrapMode", (playable, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTraversalMode<U>(
        playable: U,
        mode: crate::UnityEngine::Playables::PlayableTraversalMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTraversalMode", (playable, mode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
