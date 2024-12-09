#[cfg(feature = "UnityEngine+Playables+PlayableGraph")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayableGraph {
    pub m_Handle: crate::System::IntPtr,
    pub m_Version: u32,
}
#[cfg(feature = "UnityEngine+Playables+PlayableGraph")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableGraph =>
    "UnityEngine.Playables"."PlayableGraph"
);
#[cfg(feature = "UnityEngine+Playables+PlayableGraph")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::PlayableGraph {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableGraph")]
impl crate::UnityEngine::Playables::PlayableGraph {
    pub fn Connect<U, V>(
        &mut self,
        source: U,
        sourceOutputPort: i32,
        destination: V,
        destinationInputPort: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        V: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Connect",
            (source, sourceOutputPort, destination, destinationInputPort),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ConnectInternal(
        &mut self,
        source: crate::UnityEngine::Playables::PlayableHandle,
        sourceOutputPort: i32,
        destination: crate::UnityEngine::Playables::PlayableHandle,
        destinationInputPort: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConnectInternal",
            (source, sourceOutputPort, destination, destinationInputPort),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CreatePlayableHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreatePlayableHandle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CreateScriptOutputInternal(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableOutputHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateScriptOutputInternal",
            (name, handle),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Evaluate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Evaluate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Evaluate_f32_1(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Evaluate",
            (deltaTime),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetFrameRate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::FrameRate> {
        let __cordl_ret: crate::UnityEngine::Playables::FrameRate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetFrameRate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayableCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPlayableCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::IExposedPropertyTable> {
        let __cordl_ret: *mut crate::UnityEngine::IExposedPropertyTable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResolver",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetRootPlayable(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRootPlayable",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetRootPlayableCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRootPlayableCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetRootPlayableInternal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRootPlayableInternal",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsMatchFrameRateEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsMatchFrameRateEnabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPlaying",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SynchronizeEvaluation(
        &mut self,
        playable: crate::UnityEngine::Playables::PlayableGraph,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SynchronizeEvaluation",
            (playable),
        )?;
        Ok(__cordl_ret)
    }
}
