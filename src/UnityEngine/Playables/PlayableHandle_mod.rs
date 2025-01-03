#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlayableHandle {
    pub m_Handle: crate::System::IntPtr,
    pub m_Version: u32,
}
#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::PlayableHandle =>
    "UnityEngine.Playables"."PlayableHandle"
);
#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::PlayableHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
impl crate::UnityEngine::Playables::PlayableHandle {
    pub fn CheckInputBounds__cordl_bool1(
        &mut self,
        inputIndex: i32,
        acceptAny: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckInputBounds",
            (inputIndex, acceptAny),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckInputBounds_i32_0(
        &mut self,
        inputIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckInputBounds",
            (inputIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareVersion(
        lhs: crate::UnityEngine::Playables::PlayableHandle,
        rhs: crate::UnityEngine::Playables::PlayableHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareVersion", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_PlayableHandle1(
        &mut self,
        other: crate::UnityEngine::Playables::PlayableHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDuration(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDuration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDuration_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDuration_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableGraph> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableGraph = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetGraph",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraph_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraph_Injected", (_unity_self, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInput(
        &mut self,
        inputPort: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetInput",
            (inputPort),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetInputCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputCount_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputCount_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputHandle(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetInputHandle",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputHandle_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputHandle_Injected", (_unity_self, index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputWeight(
        &mut self,
        inputIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetInputWeight",
            (inputIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputWeightFromIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetInputWeightFromIndex",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputWeightFromIndex_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInputWeightFromIndex_Injected", (_unity_self, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObject<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetObject",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayState> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPlayState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayState_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayState> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayState_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPlayableType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayableType_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayableType_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPreviousTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousTime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviousTime_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScriptInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetScriptInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScriptInstance_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetScriptInstance_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeWrapMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::DirectorWrapMode> {
        let __cordl_ret: crate::UnityEngine::Playables::DirectorWrapMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTimeWrapMode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeWrapMode_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::DirectorWrapMode> {
        let __cordl_ret: crate::UnityEngine::Playables::DirectorWrapMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeWrapMode_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTime_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsDone",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDone_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDone_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPlayableOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPlayableOfType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Pause",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pause_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn Play(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Play",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Play_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Play_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDone(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDone",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDone_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDone_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDuration(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDuration",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDuration_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDuration_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetInputCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputCount_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputCount_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeightFromIndex(
        &mut self,
        index: i32,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetInputWeightFromIndex",
            (index, weight),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeightFromIndex_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        index: i32,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputWeightFromIndex_Injected", (_unity_self, index, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeight_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInputWeight_Injected", (_unity_self, input, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeight_PlayableHandle1(
        &mut self,
        input: crate::UnityEngine::Playables::PlayableHandle,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetInputWeight",
            (input, weight),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInputWeight_i32_0(
        &mut self,
        inputIndex: i32,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetInputWeight",
            (inputIndex, weight),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropagateSetTime(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetPropagateSetTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropagateSetTime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPropagateSetTime_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScriptInstance(
        &mut self,
        scriptInstance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetScriptInstance",
            (scriptInstance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScriptInstance_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        scriptInstance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetScriptInstance_Injected", (_unity_self, scriptInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpeed(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSpeed",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpeed_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSpeed_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTimeWrapMode(
        &mut self,
        mode: crate::UnityEngine::Playables::DirectorWrapMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTimeWrapMode",
            (mode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTimeWrapMode_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        mode: crate::UnityEngine::Playables::DirectorWrapMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTimeWrapMode_Injected", (_unity_self, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTime_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTime_Injected", (_unity_self, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTraversalMode(
        &mut self,
        mode: crate::UnityEngine::Playables::PlayableTraversalMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTraversalMode",
            (mode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTraversalMode_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        mode: crate::UnityEngine::Playables::PlayableTraversalMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTraversalMode_Injected", (_unity_self, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Null() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableHandle,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Null", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::UnityEngine::Playables::PlayableHandle,
        y: crate::UnityEngine::Playables::PlayableHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Playables::PlayableHandle>>
for crate::UnityEngine::Playables::PlayableHandle {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Playables::PlayableHandle> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableHandle")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Playables::PlayableHandle>>
for crate::UnityEngine::Playables::PlayableHandle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Playables::PlayableHandle,
    > {
        todo!()
    }
}
