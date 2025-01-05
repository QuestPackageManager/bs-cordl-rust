#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ProfilerRecorder {
    pub handle: u64,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerRecorder =>
    "Unity.Profiling"."ProfilerRecorder"
);
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerRecorder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl crate::Unity::Profiling::ProfilerRecorder {
    #[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
    pub type ControlOptions = crate::Unity::Profiling::ProfilerRecorder_ControlOptions;
    pub fn CheckInitializedAndThrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckInitializedAndThrow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Control(
        handle: crate::Unity::Profiling::ProfilerRecorder,
        options: crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Control", (handle, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Control_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
        options: crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Control_Injected", (handle, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        statHandle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        maxSampleCount: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerRecorder> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerRecorder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (statHandle, maxSampleCount, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Injected(
        statHandle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
        maxSampleCount: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Profiling::ProfilerRecorder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create_Injected", (statHandle, maxSampleCount, options, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValue(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastValue", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValue_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastValue_Injected", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValid(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValid", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValid_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValid_Injected", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueUnitType(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueUnitType", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueUnitType_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueUnitType_Injected", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartNew(
        category: crate::Unity::Profiling::ProfilerCategory,
        statName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        capacity: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerRecorder> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerRecorder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartNew", (category, statName, capacity, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        category: crate::Unity::Profiling::ProfilerCategory,
        statName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        statNameLen: i32,
        capacity: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (category, statName, statNameLen, capacity, options),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastValue(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LastValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnitType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_UnitType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::Unity::Profiling::ProfilerRecorder {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::Unity::Profiling::ProfilerRecorder {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerRecorder_ControlOptions {
    #[default]
    Release = 4i32,
    Reset = 2i32,
    SetFilterToCurrentThread = 5i32,
    SetToCollectFromAllThreads = 6i32,
    Start = 0i32,
    Stop = 1i32,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Profiling::ProfilerRecorder_ControlOptions => "Unity.Profiling"
    ."ProfilerRecorder/ControlOptions"
);
