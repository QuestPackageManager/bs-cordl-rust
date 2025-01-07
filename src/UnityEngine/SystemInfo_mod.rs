#[cfg(feature = "UnityEngine+SystemInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SystemInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SystemInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SystemInfo";
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
#[cfg(feature = "UnityEngine+SystemInfo")]
impl std::ops::Deref for crate::UnityEngine::SystemInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl std::ops::DerefMut for crate::UnityEngine::SystemInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl crate::UnityEngine::SystemInfo {
    pub fn GetBatteryLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBatteryLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBatteryStatus() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::BatteryStatus,
    > {
        let __cordl_ret: crate::UnityEngine::BatteryStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBatteryStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCompatibleFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        usage: crate::UnityEngine::Experimental::Rendering::FormatUsage,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCompatibleFormat", (format, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceModel() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetDeviceModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetDeviceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::DeviceType,
    > {
        let __cordl_ret: crate::UnityEngine::DeviceType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceUniqueIdentifier() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceUniqueIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceID() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GraphicsDeviceType,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsDeviceType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceVendor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceVendor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceVendorID() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceVendorID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsDeviceVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsDeviceVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat(
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsMemorySize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsMemorySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsMultiThreaded() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsMultiThreaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsShaderLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsShaderLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxRenderTextureSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxRenderTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxTextureSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOperatingSystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOperatingSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOperatingSystemFamily() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::OperatingSystemFamily,
    > {
        let __cordl_ret: crate::UnityEngine::OperatingSystemFamily = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOperatingSystemFamily", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicalMemoryMB() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPhysicalMemoryMB", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessorCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessorCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessorFrequencyMHz() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessorFrequencyMHz", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessorType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderingThreadingMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderingThreadingMode,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::RenderingThreadingMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderingThreadingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasRenderTextureNative(
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasRenderTextureNative", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFormatSupported(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        usage: crate::UnityEngine::Experimental::Rendering::FormatUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFormatSupported", (format, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidEnumValue(
        value: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidEnumValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsComputeShaders() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsComputeShaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsInstancing() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsInstancing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsRenderTextureFormat(
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsRenderTextureFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsTextureFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsTextureFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsTextureFormatNative(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SupportsTextureFormatNative", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn UsesReversedZBuffer() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UsesReversedZBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryStatus() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::BatteryStatus,
    > {
        let __cordl_ret: crate::UnityEngine::BatteryStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceModel() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deviceModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_deviceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::DeviceType,
    > {
        let __cordl_ret: crate::UnityEngine::DeviceType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deviceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceUniqueIdentifier() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deviceUniqueIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceID() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GraphicsDeviceType,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsDeviceType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceVendor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceVendor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceVendorID() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceVendorID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsDeviceVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsDeviceVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsMemorySize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsMemorySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsMultiThreaded() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsMultiThreaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsShaderLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_graphicsShaderLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxRenderTextureSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxRenderTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxTextureSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxTextureSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_operatingSystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_operatingSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_operatingSystemFamily() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::OperatingSystemFamily,
    > {
        let __cordl_ret: crate::UnityEngine::OperatingSystemFamily = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_operatingSystemFamily", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processorCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_processorCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processorFrequency() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_processorFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_processorType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_processorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingThreadingMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderingThreadingMode,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::RenderingThreadingMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderingThreadingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportsComputeShaders() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_supportsComputeShaders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportsInstancing() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_supportsInstancing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemMemorySize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemMemorySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usesReversedZBuffer() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_usesReversedZBuffer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SystemInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SystemInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
