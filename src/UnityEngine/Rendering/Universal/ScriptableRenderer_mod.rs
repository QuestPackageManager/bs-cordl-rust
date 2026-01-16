#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Profiling_ScriptableRenderer_RenderBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/Profiling/RenderBlock";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock")]
impl crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock {
    pub const k_Name: &'static str = "RenderPassBlock";
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Profiling_ScriptableRenderer_RenderPass {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/Profiling/RenderPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass")]
impl crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass {
    pub const k_Name: &'static str = "ScriptableRenderPass";
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RenderBlocks_ScriptableRenderer_BlockRange {
    pub m_Current: i32,
    pub m_End: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/RenderBlocks/BlockRange";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange")]
impl crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange {
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange,
                        0usize,
                    >("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MoveNext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        begin: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (begin, end))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Current",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LastBeginSubpassPassIndex: i32,
    pub m_MergeableRenderPassesMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::Hash128,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    >,
    pub m_MergeableRenderPassesMapArrays: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    >,
    pub m_PassIndexToPassHash:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Hash128>>,
    pub m_RenderPassesAttachmentCount: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<crate::UnityEngine::Hash128, i32>,
    >,
    pub m_firstPassIndexOfLastMergeableGroup: i32,
    pub m_ActiveColorAttachmentDescriptors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::AttachmentDescriptor>,
    >,
    pub m_ActiveDepthAttachmentDescriptor: crate::UnityEngine::Rendering::AttachmentDescriptor,
    pub m_IsActiveColorAttachmentTransient:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    pub m_FinalColorStoreAction: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::RenderBufferStoreAction>,
    >,
    pub m_FinalDepthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    pub _profilingExecute_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub hasReleasedRTs: bool,
    pub _DebugHandler_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DebugHandler>,
    pub _supportedRenderingFeatures_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
    >,
    pub _unsupportedGraphicsDeviceTypes_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::GraphicsDeviceType>,
    >,
    pub m_StoreActionsOptimizationSetting:
        crate::UnityEngine::Rendering::Universal::StoreActionsOptimization,
    pub m_ActiveRenderPassQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
            >,
        >,
    >,
    pub m_RendererFeatures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
            >,
        >,
    >,
    pub m_CameraColorTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_CameraDepthTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_CameraResolveTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_FirstTimeCameraColorTargetIsBound: bool,
    pub m_FirstTimeCameraDepthTargetIsBound: bool,
    pub m_IsPipelineExecuting: bool,
    pub disableNativeRenderPassInFeatures: bool,
    pub useRenderPassEnabled: bool,
    pub m_frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
    pub _useDepthPriming_k__BackingField: bool,
    pub _stripShadowsOffVariants_k__BackingField: bool,
    pub _stripAdditionalLightOffVariants_k__BackingField: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScriptableRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ScriptableRenderer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer {
    pub const kRenderPassMapSize: i32 = 10i32;
    pub const kRenderPassMaxCount: i32 = 20i32;
    pub const k_RenderPassBlockCount: i32 = 4i32;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
    pub type BeginXRPassData =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData")]
    pub type DrawGizmosPassData =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData")]
    pub type DrawWireOverlayPassData =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
    pub type DummyData = crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
    pub type EndXRPassData =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
    pub type PassData = crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
    pub type Profiling = crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
    pub type RenderBlocks =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
    pub type RenderPassBlock =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor")]
    pub type RenderPassDescriptor =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
    pub type RenderingFeatures =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData")]
    pub type VFXProcessCameraPassData =
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData;
    pub fn AddRenderPasses(
        &mut self,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::RenderingData,
                    >), quest_hook::libil2cpp::Void, 1usize>("AddRenderPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddRenderPasses",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn AdjustAndGetScreenMSAASamples(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        useIntermediateColorTarget: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        bool,
                    ), i32, 2usize>("AdjustAndGetScreenMSAASamples")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustAndGetScreenMSAASamples",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderGraph, useIntermediateColorTarget))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AreAttachmentIndicesCompatible(
        lastSubPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        currentSubPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                    ), bool, 2usize>("AreAttachmentIndicesCompatible")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AreAttachmentIndicesCompatible",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (lastSubPass, currentSubPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginRenderGraphXRRendering(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "BeginRenderGraphXRRendering"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRenderGraphXRRendering",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginXRRendering(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("BeginXRRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginXRRendering",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, context, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateBillboardProperties(
        worldToCameraMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        billboardTangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        billboardNormal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        cameraXZAngle: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        quest_hook::libil2cpp::ByRefMut<f32>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CalculateBillboardProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateBillboardProperties",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    worldToCameraMatrix,
                    billboardTangent,
                    billboardNormal,
                    cameraXZAngle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateSplitEventRange(
        &mut self,
        startInjectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        targetEvent: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        startEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        >,
        splitEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        >,
        endEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "CalculateSplitEventRange"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateSplitEventRange",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    startInjectionPoint,
                    targetEvent,
                    startEvent,
                    splitEvent,
                    endEvent,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
        cameraType: crate::UnityEngine::Rendering::Universal::CameraRenderType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::CameraRenderType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderingState(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::IBaseCommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::IBaseCommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ClearRenderingState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderingState", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCameraColorTarget(
        &mut self,
        colorTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ConfigureCameraColorTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureCameraColorTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colorTarget))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCameraTarget_RTHandle_RTHandle1(
        &mut self,
        colorTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ConfigureCameraTarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConfigureCameraTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colorTarget, depthTarget))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCameraTarget_RTHandle_RTHandle_RTHandle2(
        &mut self,
        colorTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        resolveTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ConfigureCameraTarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConfigureCameraTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (colorTarget, depthTarget, resolveTarget))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCameraTarget_RenderTargetIdentifier_RenderTargetIdentifier0(
        &mut self,
        colorTarget: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthTarget: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ConfigureCameraTarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConfigureCameraTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colorTarget, depthTarget))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRenderPassHash_ScriptableRenderer_RenderPassDescriptor_u32_1(
        desc: crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor,
        hashIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Hash128> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor,
                            u32,
                        ),
                        crate::UnityEngine::Hash128,
                        2usize,
                    >("CreateRenderPassHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRenderPassHash", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Hash128 =
            unsafe { cordl_method_info.invoke_unchecked((), (desc, hashIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRenderPassHash_i32_i32_i32_i32_u32_0(
        width: i32,
        height: i32,
        depthID: i32,
        sample: i32,
        hashIndex: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Hash128> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32, i32, u32),
                        crate::UnityEngine::Hash128,
                        5usize,
                    >("CreateRenderPassHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRenderPassHash", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Hash128 = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height, depthID, sample, hashIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disposing))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawGizmos(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        gizmoSubset: crate::UnityEngine::Rendering::GizmoSubset,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::GizmoSubset,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawGizmos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawGizmos",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (context, camera, gizmoSubset, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderGraphGizmos(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        color: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        depth: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        gizmoSubset: crate::UnityEngine::Rendering::GizmoSubset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::GizmoSubset,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawRenderGraphGizmos"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderGraphGizmos",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, frameData, color, depth, gizmoSubset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderGraphWireOverlay(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        color: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "DrawRenderGraphWireOverlay"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderGraphWireOverlay",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, frameData, color))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireOverlay(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), quest_hook::libil2cpp::Void, 2usize>("DrawWireOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawWireOverlay",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableSwapBufferMSAA(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableSwapBufferMSAA",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableSwapBufferMSAA",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (enable))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndRenderGraphXRRendering(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "EndRenderGraphXRRendering"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndRenderGraphXRRendering",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndXRRendering(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("EndXRRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndXRRendering",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, context, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnqueuePass(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), quest_hook::libil2cpp::Void, 1usize>("EnqueuePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnqueuePass",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteBlock(
        &mut self,
        blockIndex: i32,
        renderBlocks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks,
        >,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
        submit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks,
                            >,
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingData,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ExecuteBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteBlock", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (blockIndex, renderBlocks, context, renderingData, submit),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteNativeRenderPass(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ExecuteNativeRenderPass"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteNativeRenderPass",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (context, renderPass, cameraData, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteRenderPass(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("ExecuteRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteRenderPass",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (context, renderPass, cameraData, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindAttachmentDescriptorIndexInList_RenderTargetIdentifier_Il2CppArray1(
        target: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        attachmentDescriptors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::AttachmentDescriptor>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::AttachmentDescriptor,
                            >,
                        >,
                    ), i32, 2usize>("FindAttachmentDescriptorIndexInList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindAttachmentDescriptorIndexInList",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (target, attachmentDescriptors))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindAttachmentDescriptorIndexInList_i32_AttachmentDescriptor_Il2CppArray0(
        attachmentIdx: i32,
        attachmentDescriptor: crate::UnityEngine::Rendering::AttachmentDescriptor,
        attachmentDescriptors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::AttachmentDescriptor>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        crate::UnityEngine::Rendering::AttachmentDescriptor,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::AttachmentDescriptor,
                            >,
                        >,
                    ), i32, 3usize>("FindAttachmentDescriptorIndexInList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindAttachmentDescriptorIndexInList",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (attachmentIdx, attachmentDescriptor, attachmentDescriptors),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FinishRenderGraphRendering(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FinishRenderGraphRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FinishRenderGraphRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinishRendering(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FinishRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FinishRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraClearFlag_ByRefMut0(
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ClearFlag> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::CameraData,
                    >), crate::UnityEngine::Rendering::ClearFlag, 1usize>(
                        "GetCameraClearFlag"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraClearFlag",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ClearFlag =
            unsafe { cordl_method_info.invoke_unchecked((), (cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraClearFlag_UniversalCameraData1(
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ClearFlag> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                    >), crate::UnityEngine::Rendering::ClearFlag, 1usize>(
                        "GetCameraClearFlag"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraClearFlag",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ClearFlag =
            unsafe { cordl_method_info.invoke_unchecked((), (cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraColorBackBuffer(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("GetCameraColorBackBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCameraColorBackBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraColorFrontBuffer(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("GetCameraColorFrontBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCameraColorFrontBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstAllocatedRTHandle(
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 1usize>(
                        "GetFirstAllocatedRTHandle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFirstAllocatedRTHandle",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTextureDescriptor(
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        targetRT: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetRenderTextureDescriptor"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRenderTextureDescriptor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cameraData, renderPass, targetRT))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSubPassAttachmentIndicesCount(
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), u32, 1usize>("GetSubPassAttachmentIndicesCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSubPassAttachmentIndicesCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidColorAttachmentCount(
        colorAttachments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::AttachmentDescriptor>,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::Rendering::AttachmentDescriptor,
                        >,
                    >), u32, 1usize>("GetValidColorAttachmentCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetValidColorAttachmentCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (colorAttachments))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidInputAttachmentCount(
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), i32, 1usize>("GetValidInputAttachmentCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetValidInputAttachmentCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (renderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidPassIndexCount(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >),
                        i32,
                        1usize,
                    >("GetValidPassIndexCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetValidPassIndexCount", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitRenderGraphFrame(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "InitRenderGraphFrame"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitRenderGraphFrame",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeRenderPassDescriptor(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                            >,
                        ),
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor,
                        2usize,
                    >("InitializeRenderPassDescriptor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeRenderPassDescriptor", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor = unsafe {
            cordl_method_info.invoke_unchecked(self, (cameraData, renderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalFinishRenderingCommon(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        resolveFinalTarget: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalFinishRenderingCommon"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalFinishRenderingCommon",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, resolveFinalTarget))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalFinishRenderingExecute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        resolveFinalTarget: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InternalFinishRenderingExecute"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalFinishRenderingExecute",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, cmd, resolveFinalTarget))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalStartRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalStartRendering"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalStartRendering",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn InterruptFramebufferFetch(
        &mut self,
        fetchEvent: crate::UnityEngine::Rendering::Universal::FramebufferFetchEvent,
        startInjectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        endInjectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::Universal::FramebufferFetchEvent,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                    ), bool, 3usize>("InterruptFramebufferFetch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InterruptFramebufferFetch",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (fetchEvent, startInjectionPoint, endInjectionPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDepthOnlyRenderTexture(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                        bool,
                        1usize,
                    >("IsDepthOnlyRenderTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDepthOnlyRenderTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (t))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsRenderPassEnabled(
        &mut self,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), bool, 1usize>("IsRenderPassEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsRenderPassEnabled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (renderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneFilteringEnabled(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        bool,
                        1usize,
                    >("IsSceneFilteringEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSceneFilteringEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRendererData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginRenderGraphFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OnBeginRenderGraphFrame",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnBeginRenderGraphFrame",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndRenderGraphFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEndRenderGraphFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEndRenderGraphFrame",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnFinishRenderGraphRendering(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnFinishRenderGraphRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnFinishRenderGraphRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnPreCullRenderPasses(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::CameraData,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "OnPreCullRenderPasses"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnPreCullRenderPasses",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnRecordRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "OnRecordRenderGraph"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnRecordRenderGraph",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn PassHasInputAttachments(
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), bool, 1usize>("PassHasInputAttachments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PassHasInputAttachments",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (renderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessVFXCameraCommand(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "ProcessVFXCameraCommand"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessVFXCameraCommand",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordCustomRenderGraphPassesInEventRange(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        eventStart: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        eventEnd: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RecordCustomRenderGraphPassesInEventRange",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordCustomRenderGraphPassesInEventRange",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderGraph, eventStart, eventEnd))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecordCustomRenderGraphPasses_RenderGraph_RenderPassEvent1(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        injectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "RecordCustomRenderGraphPasses"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordCustomRenderGraphPasses",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, injectionPoint))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordCustomRenderGraphPasses_RenderPassEvent0(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        startInjectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        endInjectionPoint: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RecordCustomRenderGraphPasses"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordCustomRenderGraphPasses",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, startInjectionPoint, endInjectionPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecordRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                    ), quest_hook::libil2cpp::Void, 2usize>("RecordRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordRenderGraph",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseRenderTargets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ReleaseRenderTargets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseRenderTargets",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetNativeRenderPassFrameData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ResetNativeRenderPassFrameData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetNativeRenderPassFrameData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraMatrices_CommandBuffer_ByRefMut0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        setInverseMatrices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetCameraMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetCameraMatrices",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, cameraData, setInverseMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraMatrices_CommandBuffer_UniversalCameraData1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        setInverseMatrices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetCameraMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetCameraMatrices",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, cameraData, setInverseMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraMatrices_RasterCommandBuffer_UniversalCameraData__cordl_bool2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        setInverseMatrices: bool,
        isTargetFlipped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetCameraMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetCameraMatrices",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, cameraData, setInverseMatrices, isTargetFlipped))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetEditorTarget(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), quest_hook::libil2cpp::Void, 1usize>("SetEditorTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetEditorTarget",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetNativeRenderPassAttachmentList(
        &mut self,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        passColorAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        passDepthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        finalClearFlag: crate::UnityEngine::Rendering::ClearFlag,
        finalClearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetNativeRenderPassAttachmentList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetNativeRenderPassAttachmentList",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderPass,
                    cameraData,
                    passColorAttachment,
                    passDepthAttachment,
                    finalClearFlag,
                    finalClearColor,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNativeRenderPassMRTAttachmentList(
        &mut self,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        needCustomCameraColorClear: bool,
        cameraClearFlag: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        bool,
                        crate::UnityEngine::Rendering::ClearFlag,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetNativeRenderPassMRTAttachmentList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetNativeRenderPassMRTAttachmentList",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderPass,
                    cameraData,
                    needCustomCameraColorClear,
                    cameraClearFlag,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraBillboardProperties(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetPerCameraBillboardProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraBillboardProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraClippingPlaneProperties_ByRefMut__cordl_bool1(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
            >,
        >,
        isTargetFlipped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetPerCameraClippingPlaneProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraClippingPlaneProperties",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cmd, cameraData, isTargetFlipped))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraClippingPlaneProperties_UniversalCameraData0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetPerCameraClippingPlaneProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraClippingPlaneProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraProperties(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetPerCameraProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraProperties",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, cameraData, camera, cmd))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraShaderVariables_RasterCommandBuffer_UniversalCameraData0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetPerCameraShaderVariables"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraShaderVariables",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPerCameraShaderVariables_Vector2Int__cordl_bool1(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        cameraTargetSizeCopy: crate::UnityEngine::Vector2Int,
        isTargetFlipped: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        crate::UnityEngine::Vector2Int,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetPerCameraShaderVariables"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPerCameraShaderVariables",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, cameraData, cameraTargetSizeCopy, isTargetFlipped),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderPassAttachments(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        renderPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRenderPassAttachments"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderPassAttachments",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, renderPass, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_Il2CppArray_RTHandle_ClearFlag_Color3(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorAttachments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
        colorAttachmentIDs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 6usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    colorAttachments,
                    colorAttachmentIDs,
                    depthAttachment,
                    clearFlag,
                    clearColor,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RTHandle_ClearFlag_Color0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (cmd, colorAttachment, depthAttachment, clearFlag, clearColor),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RTHandle_ClearFlag_Color_RenderBufferStoreAction_RenderBufferStoreAction1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 7usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    colorAttachment,
                    depthAttachment,
                    clearFlag,
                    clearColor,
                    colorStoreAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        colorAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlags: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 9usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    colorAttachment,
                    colorLoadAction,
                    colorStoreAction,
                    depthAttachment,
                    depthLoadAction,
                    depthStoreAction,
                    clearFlags,
                    clearColor,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShaderTimeValues(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::IBaseCommandBuffer>,
        _cordl_time: f32,
        deltaTime: f32,
        smoothDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::IBaseCommandBuffer,
                        >,
                        f32,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetShaderTimeValues"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetShaderTimeValues",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, _cordl_time, deltaTime, smoothDeltaTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCullingParameters(
        &mut self,
        cullingParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableCullingParameters,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupCullingParameters"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupCullingParameters",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cullingParameters, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupInputAttachmentIndices(
        &mut self,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "SetupInputAttachmentIndices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupInputAttachmentIndices",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupLights(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupLights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupLights",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupNativeRenderPassFrameData(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        isRenderPassEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupNativeRenderPassFrameData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupNativeRenderPassFrameData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData, isRenderPassEnabled))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupRenderGraphCameraProperties(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        isTargetBackbuffer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupRenderGraphCameraProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupRenderGraphCameraProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, isTargetBackbuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupRenderPasses(
        &mut self,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::RenderingData,
                    >), quest_hook::libil2cpp::Void, 1usize>("SetupRenderPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupRenderPasses",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupTransientInputAttachments(
        &mut self,
        attachmentCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "SetupTransientInputAttachments",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupTransientInputAttachments",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attachmentCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn SortStable(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("SortStable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SortStable",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportedCameraStackingTypes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("SupportedCameraStackingTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SupportedCameraStackingTypes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsCameraStackingType(
        &mut self,
        cameraRenderType: crate::UnityEngine::Rendering::Universal::CameraRenderType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::CameraRenderType),
                        bool,
                        1usize,
                    >("SupportsCameraStackingType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SupportsCameraStackingType", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraRenderType))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsMotionVectors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("SupportsMotionVectors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SupportsMotionVectors",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SwapColorBuffer(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SwapColorBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SwapColorBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFinalStoreActions(
        &mut self,
        currentMergeablePasses: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        isLastMergeableGroup: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateFinalStoreActions"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateFinalStoreActions",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (currentMergeablePasses, cameraData, isLastMergeableGroup),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRendererData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::ScriptableRendererData,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DebugHandler>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::DebugHandler,
                    >, 0usize>("get_DebugHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_DebugHandler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugHandler,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeRenderPassQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                            >,
                        >,
                    >, 0usize>("get_activeRenderPassQueue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_activeRenderPassQueue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraColorTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderTargetIdentifier> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        0usize,
                    >("get_cameraColorTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cameraColorTarget", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraColorTargetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        0usize,
                    >("get_cameraColorTargetHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cameraColorTargetHandle", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraDepth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderTargetIdentifier> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        0usize,
                    >("get_cameraDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cameraDepth", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraDepthTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderTargetIdentifier> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        0usize,
                    >("get_cameraDepthTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cameraDepthTarget", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraDepthTargetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        0usize,
                    >("get_cameraDepthTargetHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cameraDepthTargetHandle", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_frameData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ContextContainer,
                        >,
                        0usize,
                    >("get_frameData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_frameData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ContextContainer,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_profilingExecute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProfilingSampler,
                        >,
                        0usize,
                    >("get_profilingExecute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_profilingExecute", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProfilingSampler,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rendererFeatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                            >,
                        >,
                    >, 0usize>("get_rendererFeatures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_rendererFeatures",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stripAdditionalLightOffVariants(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_stripAdditionalLightOffVariants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stripAdditionalLightOffVariants",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stripShadowsOffVariants(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_stripShadowsOffVariants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stripShadowsOffVariants",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedRenderingFeatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
                        >,
                        0usize,
                    >("get_supportedRenderingFeatures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_supportedRenderingFeatures", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_supportsGPUOcclusion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_supportsGPUOcclusion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_supportsGPUOcclusion",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_supportsNativeRenderPassRendergraphCompiler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>(
                        "get_supportsNativeRenderPassRendergraphCompiler",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_supportsNativeRenderPassRendergraphCompiler",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_unsupportedGraphicsDeviceTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::GraphicsDeviceType>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::Rendering::GraphicsDeviceType,
                        >,
                    >, 0usize>("get_unsupportedGraphicsDeviceTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_unsupportedGraphicsDeviceTypes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::GraphicsDeviceType>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useDepthPriming(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_useDepthPriming")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_useDepthPriming",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_profilingExecute(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProfilingSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_profilingExecute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_profilingExecute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_stripAdditionalLightOffVariants(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_stripAdditionalLightOffVariants",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_stripAdditionalLightOffVariants",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_stripShadowsOffVariants(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_stripShadowsOffVariants",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_stripShadowsOffVariants",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_supportedRenderingFeatures(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_supportedRenderingFeatures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_supportedRenderingFeatures", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_unsupportedGraphicsDeviceTypes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::GraphicsDeviceType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::Rendering::GraphicsDeviceType,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_unsupportedGraphicsDeviceTypes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_unsupportedGraphicsDeviceTypes",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_useDepthPriming(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_useDepthPriming",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_useDepthPriming",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_BeginXRPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/BeginXRPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+BeginXRPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_BeginXRPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_DrawGizmosPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub gizmoRenderList: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/DrawGizmosPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawGizmosPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawGizmosPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_DrawWireOverlayPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub wireOverlayList: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/DrawWireOverlayPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DrawWireOverlayPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DrawWireOverlayPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_DummyData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/DummyData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+DummyData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_DummyData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_EndXRPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/EndXRPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+EndXRPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_EndXRPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub renderer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScriptableRenderer>,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub isTargetBackbuffer: bool,
    pub cameraTargetSizeCopy: crate::UnityEngine::Vector2Int,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/PassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+PassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_PassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_Profiling {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/Profiling";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling {
    pub const k_Name: &'static str = "ScriptableRenderer";
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderBlock")]
    pub type RenderBlock =
        crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderBlock;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling+RenderPass")]
    pub type RenderPass =
        crate::UnityEngine::Rendering::Universal::Profiling_ScriptableRenderer_RenderPass;
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+Profiling")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_Profiling
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScriptableRenderer_RenderBlocks {
    pub m_BlockEventLimits: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    >,
    pub m_BlockRanges: crate::Unity::Collections::NativeArray_1<i32>,
    pub m_BlockRangeLengths: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/RenderBlocks";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks {
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks+BlockRange")]
    pub type BlockRange =
        crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange;
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FillBlockRanges(
        &mut self,
        activeRenderPassQueue: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("FillBlockRanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FillBlockRanges",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (activeRenderPassQueue))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLength(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLength",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRange(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange,
                        1usize,
                    >("GetRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRange", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::RenderBlocks_ScriptableRenderer_BlockRange = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        activeRenderPassQueue: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (activeRenderPassQueue))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderBlocks")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderBlocks
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_RenderPassBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/RenderPassBlock";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassBlock")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassBlock
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScriptableRenderer_RenderPassDescriptor {
    pub w: i32,
    pub h: i32,
    pub samples: i32,
    pub depthID: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/RenderPassDescriptor";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderPassDescriptor")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderPassDescriptor {
    pub fn _ctor(
        &mut self,
        width: i32,
        height: i32,
        sampleCount: i32,
        rtID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, i32, i32), quest_hook::libil2cpp::Void, 4usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (width, height, sampleCount, rtID))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_RenderingFeatures {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cameraStacking_k__BackingField: bool,
    pub _msaa_k__BackingField: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/RenderingFeatures";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_cameraStacking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_cameraStacking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_cameraStacking",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_msaa(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_msaa")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_msaa",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_cameraStacking(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_cameraStacking",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_cameraStacking",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_msaa(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_msaa")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_msaa",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+RenderingFeatures")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_RenderingFeatures
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRenderer_VFXProcessCameraPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub renderingData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalRenderingData>,
    pub camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub cameraXRSettings: crate::UnityEngine::VFX::VFXCameraXRSettings,
    pub xrPass: quest_hook::libil2cpp::Gc<crate::UnityEngine::Experimental::Rendering::XRPass>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScriptableRenderer/VFXProcessCameraPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData")]
impl crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScriptableRenderer+VFXProcessCameraPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScriptableRenderer_VFXProcessCameraPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
