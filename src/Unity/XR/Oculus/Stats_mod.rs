#[cfg(feature = "Unity+XR+Oculus+Stats")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Stats {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Stats";
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
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl crate::Unity::XR::Oculus::Stats {
    #[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
    pub type AdaptivePerformance = crate::Unity::XR::Oculus::Stats_AdaptivePerformance;
    #[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
    pub type AppMetrics = crate::Unity::XR::Oculus::Stats_AppMetrics;
    #[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
    pub type PerfMetrics = crate::Unity::XR::Oculus::Stats_PerfMetrics;
    pub fn GetOculusDisplaySubsystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::IntegratedSubsystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOculusDisplaySubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PluginVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PluginVersion", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_AdaptivePerformance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Stats/AdaptivePerformance";
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
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    pub fn get_AdaptivePerformanceScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AdaptivePerformanceScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BatteryLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_BatteryLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BatteryTemp() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_BatteryTemp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CPULevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CPULevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPUAppTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPUAppTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPUCompositorTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPUCompositorTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPULevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPULevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MotionToPhoton() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MotionToPhoton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PowerSavingMode() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PowerSavingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RefreshRate() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RefreshRate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_AppMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Stats_AppMetrics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Stats/AppMetrics";
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
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_AppMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_AppMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl crate::Unity::XR::Oculus::Stats_AppMetrics {
    pub fn EnableAppMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableAppMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AppCPUElapsedTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AppCPUElapsedTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AppQueueAheadTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AppQueueAheadTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CPUStartToGPUEnd() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CPUStartToGPUEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompositorCPUTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompositorCPUTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompositorDroppedFrames() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompositorDroppedFrames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompositorLatency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompositorLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPUEndToVsync() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPUEndToVsync", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats_AppMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_PerfMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Stats/PerfMetrics";
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
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl crate::Unity::XR::Oculus::Stats_PerfMetrics {
    pub fn EnablePerfMetrics(
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnablePerfMetrics", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AppCPUTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AppCPUTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AppGPUTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AppGPUTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CPUClockFrequency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CPUClockFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CPUUtilizationAverage() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CPUUtilizationAverage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CPUUtilizationWorst() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CPUUtilizationWorst", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompositorCPUTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompositorCPUTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompositorGPUTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompositorGPUTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPUClockFrequency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPUClockFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GPUUtilization() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GPUUtilization", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
