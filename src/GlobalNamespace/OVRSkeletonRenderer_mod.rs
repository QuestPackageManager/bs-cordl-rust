#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonRenderer_BoneVisualization {
    __cordl_parent: crate::System::Object,
    pub BoneGO: *mut crate::UnityEngine::GameObject,
    pub BoneBegin: *mut crate::UnityEngine::Transform,
    pub BoneEnd: *mut crate::UnityEngine::Transform,
    pub Line: *mut crate::UnityEngine::LineRenderer,
    pub RenderMaterial: *mut crate::UnityEngine::Material,
    pub SystemGestureMaterial: *mut crate::UnityEngine::Material,
}
#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization => ""
    ."OVRSkeletonRenderer/BoneVisualization"
);
#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
impl crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization {
    pub fn New(
        rootGO: *mut crate::UnityEngine::GameObject,
        renderMat: *mut crate::UnityEngine::Material,
        systemGestureMat: *mut crate::UnityEngine::Material,
        scale: f32,
        begin: *mut crate::UnityEngine::Transform,
        end: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (rootGO, renderMat, systemGestureMat, scale, begin, end),
            )?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
        scale: f32,
        shouldRender: bool,
        shouldUseSystemGestureMaterial: bool,
        confidenceBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_ConfidenceBehavior,
        systemGestureBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_SystemGestureBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Update",
                (
                    scale,
                    shouldRender,
                    shouldUseSystemGestureMaterial,
                    confidenceBehavior,
                    systemGestureBehavior,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rootGO: *mut crate::UnityEngine::GameObject,
        renderMat: *mut crate::UnityEngine::Material,
        systemGestureMat: *mut crate::UnityEngine::Material,
        scale: f32,
        begin: *mut crate::UnityEngine::Transform,
        end: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rootGO, renderMat, systemGestureMat, scale, begin, end))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonRenderer_CapsuleVisualization {
    __cordl_parent: crate::System::Object,
    pub CapsuleGO: *mut crate::UnityEngine::GameObject,
    pub BoneCapsule: *mut OVRBoneCapsule,
    pub capsuleScale: crate::UnityEngine::Vector3,
    pub Renderer: *mut crate::UnityEngine::MeshRenderer,
    pub RenderMaterial: *mut crate::UnityEngine::Material,
    pub SystemGestureMaterial: *mut crate::UnityEngine::Material,
}
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization => ""
    ."OVRSkeletonRenderer/CapsuleVisualization"
);
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
impl crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization {
    pub fn New(
        rootGO: *mut crate::UnityEngine::GameObject,
        renderMat: *mut crate::UnityEngine::Material,
        systemGestureMat: *mut crate::UnityEngine::Material,
        scale: f32,
        boneCapsule: *mut OVRBoneCapsule,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (rootGO, renderMat, systemGestureMat, scale, boneCapsule),
            )?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
        scale: f32,
        shouldRender: bool,
        shouldUseSystemGestureMaterial: bool,
        confidenceBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_ConfidenceBehavior,
        systemGestureBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_SystemGestureBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Update",
                (
                    scale,
                    shouldRender,
                    shouldUseSystemGestureMaterial,
                    confidenceBehavior,
                    systemGestureBehavior,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rootGO: *mut crate::UnityEngine::GameObject,
        renderMat: *mut crate::UnityEngine::Material,
        systemGestureMat: *mut crate::UnityEngine::Material,
        scale: f32,
        boneCapsule: *mut OVRBoneCapsule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rootGO, renderMat, systemGestureMat, scale, boneCapsule))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+ConfidenceBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRSkeletonRenderer_ConfidenceBehavior {
    None = 0i32,
    ToggleRenderer = 1i32,
}
#[cfg(feature = "OVRSkeletonRenderer+ConfidenceBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_ConfidenceBehavior => ""
    ."OVRSkeletonRenderer/ConfidenceBehavior"
);
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider => ""
    ."OVRSkeletonRenderer/IOVRSkeletonRendererDataProvider"
);
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
impl crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
    pub fn GetSkeletonRendererData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData = __cordl_object
            .invoke("GetSkeletonRendererData", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeletonRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSkeletonRenderer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _dataProvider: *mut crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider,
    pub _confidenceBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_ConfidenceBehavior,
    pub _systemGestureBehavior: crate::GlobalNamespace::OVRSkeletonRenderer_SystemGestureBehavior,
    pub _renderPhysicsCapsules: bool,
    pub _skeletonMaterial: *mut crate::UnityEngine::Material,
    pub _skeletonDefaultMaterial: *mut crate::UnityEngine::Material,
    pub _capsuleMaterial: *mut crate::UnityEngine::Material,
    pub _capsuleDefaultMaterial: *mut crate::UnityEngine::Material,
    pub _systemGestureMaterial: *mut crate::UnityEngine::Material,
    pub _systemGestureDefaultMaterial: *mut crate::UnityEngine::Material,
    pub _boneVisualizations: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization,
    >,
    pub _capsuleVisualizations: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization,
    >,
    pub _ovrSkeleton: *mut OVRSkeleton,
    pub _skeletonGO: *mut crate::UnityEngine::GameObject,
    pub _scale: f32,
    pub _IsInitialized_k__BackingField: bool,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _ShouldUseSystemGestureMaterial_k__BackingField: bool,
}
#[cfg(feature = "OVRSkeletonRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRSkeletonRenderer => ""."OVRSkeletonRenderer"
);
#[cfg(feature = "OVRSkeletonRenderer")]
impl std::ops::Deref for OVRSkeletonRenderer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer")]
impl std::ops::DerefMut for OVRSkeletonRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSkeletonRenderer")]
impl OVRSkeletonRenderer {
    pub const LINE_RENDERER_WIDTH: f32 = 0.005f32;
    #[cfg(feature = "OVRSkeletonRenderer+ConfidenceBehavior")]
    pub type ConfidenceBehavior = crate::GlobalNamespace::OVRSkeletonRenderer_ConfidenceBehavior;
    #[cfg(feature = "OVRSkeletonRenderer+IOVRSkeletonRendererDataProvider")]
    type IOVRSkeletonRendererDataProvider = crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider;
    #[cfg(feature = "OVRSkeletonRenderer+SkeletonRendererData")]
    pub type SkeletonRendererData = crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData;
    #[cfg(feature = "OVRSkeletonRenderer+SystemGestureBehavior")]
    pub type SystemGestureBehavior = crate::GlobalNamespace::OVRSkeletonRenderer_SystemGestureBehavior;
    #[cfg(feature = "OVRSkeletonRenderer+BoneVisualization")]
    pub type BoneVisualization = crate::GlobalNamespace::OVRSkeletonRenderer_BoneVisualization;
    #[cfg(feature = "OVRSkeletonRenderer+CapsuleVisualization")]
    pub type CapsuleVisualization = crate::GlobalNamespace::OVRSkeletonRenderer_CapsuleVisualization;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldInitialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldInitialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataHighConfidence", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldUseSystemGestureMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldUseSystemGestureMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDataHighConfidence(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDataHighConfidence", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDataValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDataValid", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInitialized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ShouldUseSystemGestureMaterial(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ShouldUseSystemGestureMaterial", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSkeletonRenderer")]
impl quest_hook::libil2cpp::ObjectType for OVRSkeletonRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+SkeletonRendererData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSkeletonRenderer_SkeletonRendererData {
    pub _RootScale_k__BackingField: f32,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _ShouldUseSystemGestureMaterial_k__BackingField: bool,
}
#[cfg(feature = "OVRSkeletonRenderer+SkeletonRendererData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData => ""
    ."OVRSkeletonRenderer/SkeletonRendererData"
);
#[cfg(feature = "OVRSkeletonRenderer+SkeletonRendererData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSkeletonRenderer+SkeletonRendererData")]
impl crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData {
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsDataHighConfidence",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsDataValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_RootScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootScale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldUseSystemGestureMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ShouldUseSystemGestureMaterial",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDataHighConfidence(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_IsDataHighConfidence",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDataValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_IsDataValid",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_RootScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootScale",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_ShouldUseSystemGestureMaterial(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ShouldUseSystemGestureMaterial",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSkeletonRenderer+SystemGestureBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRSkeletonRenderer_SystemGestureBehavior {
    None = 0i32,
    SwapMaterial = 1i32,
}
#[cfg(feature = "OVRSkeletonRenderer+SystemGestureBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSkeletonRenderer_SystemGestureBehavior => ""
    ."OVRSkeletonRenderer/SystemGestureBehavior"
);