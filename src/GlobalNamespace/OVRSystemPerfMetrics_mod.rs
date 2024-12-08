#[cfg(feature = "OVRSystemPerfMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSystemPerfMetrics {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRSystemPerfMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRSystemPerfMetrics => ""."OVRSystemPerfMetrics"
);
#[cfg(feature = "OVRSystemPerfMetrics")]
impl std::ops::Deref for OVRSystemPerfMetrics {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics")]
impl std::ops::DerefMut for OVRSystemPerfMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics")]
impl OVRSystemPerfMetrics {
    pub const MaxBufferLength: i32 = 65536i32;
    pub const MaxMessageLength: i32 = 65532i32;
    pub const PayloadTypeMetrics: i32 = 100i32;
    pub const TcpListeningPort: i32 = 32419i32;
    #[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
    pub type OVRSystemPerfMetricsTcpServer = crate::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer;
    #[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
    pub type PerfMetrics = crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSystemPerfMetrics")]
impl quest_hook::libil2cpp::ObjectType for OVRSystemPerfMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub tcpServer: *mut crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer,
    pub listeningPort: i32,
}
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer => ""
    ."OVRSystemPerfMetrics/OVRSystemPerfMetricsTcpServer"
);
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
impl crate::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer {
    pub fn GatherPerfMetrics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics = __cordl_object
            .invoke("GatherPerfMetrics", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+OVRSystemPerfMetricsTcpServer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSystemPerfMetrics_OVRSystemPerfMetricsTcpServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSystemPerfMetrics_PerfMetrics {
    __cordl_parent: crate::System::Object,
    pub frameCount: i32,
    pub frameTime: f32,
    pub deltaFrameTime: f32,
    pub appCpuTime_IsValid: bool,
    pub appCpuTime: f32,
    pub appGpuTime_IsValid: bool,
    pub appGpuTime: f32,
    pub compositorCpuTime_IsValid: bool,
    pub compositorCpuTime: f32,
    pub compositorGpuTime_IsValid: bool,
    pub compositorGpuTime: f32,
    pub compositorDroppedFrameCount_IsValid: bool,
    pub compositorDroppedFrameCount: i32,
    pub compositorSpaceWarpMode_IsValid: bool,
    pub compositorSpaceWarpMode: i32,
    pub systemGpuUtilPercentage_IsValid: bool,
    pub systemGpuUtilPercentage: f32,
    pub systemCpuUtilAveragePercentage_IsValid: bool,
    pub systemCpuUtilAveragePercentage: f32,
    pub systemCpuUtilWorstPercentage_IsValid: bool,
    pub systemCpuUtilWorstPercentage: f32,
    pub deviceCpuClockFrequencyInMHz_IsValid: bool,
    pub deviceCpuClockFrequencyInMHz: f32,
    pub deviceGpuClockFrequencyInMHz_IsValid: bool,
    pub deviceGpuClockFrequencyInMHz: f32,
    pub deviceCpuClockLevel_IsValid: bool,
    pub deviceCpuClockLevel: i32,
    pub deviceGpuClockLevel_IsValid: bool,
    pub deviceGpuClockLevel: i32,
    pub deviceCpuCoreUtilPercentage_IsValid: *mut quest_hook::libil2cpp::Il2CppArray<
        bool,
    >,
    pub deviceCpuCoreUtilPercentage: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
}
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics => ""
    ."OVRSystemPerfMetrics/PerfMetrics"
);
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
impl crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics {
    pub fn LoadFromJSON(
        &mut self,
        json: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadFromJSON", (json))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToJSON(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToJSON", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSystemPerfMetrics+PerfMetrics")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSystemPerfMetrics_PerfMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
