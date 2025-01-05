#[cfg(feature = "ProfilerMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfilerMetrics {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _metrics: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
        >,
    >,
    pub _profilerRecorders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
    >,
    pub _samples: quest_hook::libil2cpp::Gc<i64>,
}
#[cfg(feature = "ProfilerMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ProfilerMetrics => ""
    ."ProfilerMetrics"
);
#[cfg(feature = "ProfilerMetrics")]
impl std::ops::Deref for crate::GlobalNamespace::ProfilerMetrics {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl std::ops::DerefMut for crate::GlobalNamespace::ProfilerMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl crate::GlobalNamespace::ProfilerMetrics {
    pub const kExposedMetricsCap: i32 = 256i32;
    #[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
    pub type ProfilerMetric = crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric;
    pub fn AddExposedMetrics(
        list: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddExposedMetrics", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn CaptureFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CaptureFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateListReport(
        &mut self,
        units: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GenerateListReport", (units))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTableReport(
        &mut self,
        units: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GenerateTableReport", (units))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        profilerRecorders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (metrics, profilerRecorders, initialFrameCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn Record(
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Record", (metrics, initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordAllMetrics(
        initialFrameCapacity: i32,
        listed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordAllMetrics", (initialFrameCapacity, listed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordFrameTimingMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordFrameTimingMetrics", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordListedMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordListedMetrics", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordMemoryMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordMemoryMetrics", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordRenderingMetrics(
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ProfilerMetrics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ProfilerMetrics,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecordRenderingMetrics", (initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        metrics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric,
            >,
        >,
        profilerRecorders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Unity::Profiling::ProfilerRecorder>,
        >,
        initialFrameCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (metrics, profilerRecorders, initialFrameCapacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ProfilerMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::ProfilerMetrics {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ProfilerMetrics")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::ProfilerMetrics {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ProfilerMetrics_ProfilerMetric {
    pub category: crate::Unity::Profiling::ProfilerCategory,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ProfilerMetrics_ProfilerMetric
    => ""."ProfilerMetrics/ProfilerMetric"
);
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ProfilerMetrics+ProfilerMetric")]
impl crate::GlobalNamespace::ProfilerMetrics_ProfilerMetric {}
