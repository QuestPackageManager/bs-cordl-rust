#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeDebug")]
#[repr(C)]
#[derive(Debug)]
pub struct ProbeVolumeDebug {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub drawProbes: bool,
    pub drawBricks: bool,
    pub drawCells: bool,
    pub realtimeSubdivision: bool,
    pub subdivisionCellUpdatePerFrame: i32,
    pub subdivisionDelayInSeconds: f32,
    pub probeShading: crate::UnityEngine::Rendering::DebugProbeShadingMode,
    pub probeSize: f32,
    pub subdivisionViewCullingDistance: f32,
    pub probeCullingDistance: f32,
    pub maxSubdivToVisualize: i32,
    pub minSubdivToVisualize: i32,
    pub exposureCompensation: f32,
    pub drawProbeSamplingDebug: bool,
    pub probeSamplingDebugSize: f32,
    pub debugWithSamplingNoise: bool,
    pub samplingRenderingLayer: u32,
    pub drawVirtualOffsetPush: bool,
    pub offsetSize: f32,
    pub freezeStreaming: bool,
    pub displayCellStreamingScore: bool,
    pub displayIndexFragmentation: bool,
    pub otherStateIndex: i32,
    pub verboseStreamingLog: bool,
    pub debugStreaming: bool,
    pub autoDrawProbes: bool,
    pub isolationProbeDebug: bool,
    pub visibleLayers: u8,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeDebug")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::ProbeVolumeDebug {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeDebug";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeDebug")]
impl std::ops::Deref for crate::UnityEngine::Rendering::ProbeVolumeDebug {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeDebug")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::ProbeVolumeDebug {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeDebug")]
impl crate::UnityEngine::Rendering::ProbeVolumeDebug {
    pub fn GetReset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Action>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<crate::System::Action>, 0usize>(
                        "GetReset",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetReset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _GetReset_b__32_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("<GetReset>b__32_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<GetReset>b__32_0",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeDebug")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::ProbeVolumeDebug {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeDebug")]
impl AsRef<crate::UnityEngine::Rendering::IDebugData>
    for crate::UnityEngine::Rendering::ProbeVolumeDebug
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IDebugData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeDebug")]
impl AsMut<crate::UnityEngine::Rendering::IDebugData>
    for crate::UnityEngine::Rendering::ProbeVolumeDebug
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IDebugData {
        unsafe { std::mem::transmute(self) }
    }
}
