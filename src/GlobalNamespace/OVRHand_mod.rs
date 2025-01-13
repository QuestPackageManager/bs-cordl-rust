#[cfg(feature = "OVRHand")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHand {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub HandType: crate::GlobalNamespace::OVRHand_Hand,
    pub _pointerPoseRoot: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub m_showState: crate::GlobalNamespace::OVRInput_InputDeviceShowState,
    pub _pointerPoseGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _handState: crate::GlobalNamespace::OVRPlugin_HandState,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _IsTracked_k__BackingField: bool,
    pub _IsSystemGestureInProgress_k__BackingField: bool,
    pub _IsPointerPoseValid_k__BackingField: bool,
    pub _PointerPose_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _HandScale_k__BackingField: f32,
    pub _HandConfidence_k__BackingField: crate::GlobalNamespace::OVRHand_TrackingConfidence,
    pub _IsDominantHand_k__BackingField: bool,
}
#[cfg(feature = "OVRHand")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRHand {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRHand";
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
#[cfg(feature = "OVRHand")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHand {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHand")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHand")]
impl crate::GlobalNamespace::OVRHand {
    #[cfg(feature = "OVRHand+Hand")]
    pub type Hand = crate::GlobalNamespace::OVRHand_Hand;
    #[cfg(feature = "OVRHand+HandFinger")]
    pub type HandFinger = crate::GlobalNamespace::OVRHand_HandFinger;
    #[cfg(feature = "OVRHand+TrackingConfidence")]
    pub type TrackingConfidence = crate::GlobalNamespace::OVRHand_TrackingConfidence;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFingerConfidence(
        &mut self,
        finger: crate::GlobalNamespace::OVRHand_HandFinger,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRHand_TrackingConfidence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRHand_TrackingConfidence = __cordl_object
            .invoke("GetFingerConfidence", (finger))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFingerIsPinching(
        &mut self,
        finger: crate::GlobalNamespace::OVRHand_HandFinger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetFingerIsPinching", (finger))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFingerPinchStrength(
        &mut self,
        finger: crate::GlobalNamespace::OVRHand_HandFinger,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFingerPinchStrength", (finger))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandState(
        &mut self,
        step: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetHandState", (step))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OVRMeshRenderer_IOVRMeshRendererDataProvider_GetMeshRendererData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRMeshRenderer_MeshRendererData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMeshRenderer_MeshRendererData = __cordl_object
            .invoke(
                "OVRMeshRenderer.IOVRMeshRendererDataProvider.GetMeshRendererData",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMesh_IOVRMeshDataProvider_GetMeshType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMesh_MeshType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMesh_MeshType = __cordl_object
            .invoke("OVRMesh.IOVRMeshDataProvider.GetMeshType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider_GetSkeletonRendererData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeletonRenderer_SkeletonRendererData = __cordl_object
            .invoke(
                "OVRSkeletonRenderer.IOVRSkeletonRendererDataProvider.GetSkeletonRendererData",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_GetSkeletonPoseData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonPoseData = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.GetSkeletonPoseData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_GetSkeletonType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSkeleton_SkeletonType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSkeleton_SkeletonType = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.GetSkeletonType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_get_enabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHandType(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRHand_Hand,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHandType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_HandConfidence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRHand_TrackingConfidence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRHand_TrackingConfidence = __cordl_object
            .invoke("get_HandConfidence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_HandScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataHighConfidence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDominantHand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDominantHand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPointerPoseValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPointerPoseValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSystemGestureInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsSystemGestureInProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTracked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PointerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_PointerPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HandConfidence(
        &mut self,
        value: crate::GlobalNamespace::OVRHand_TrackingConfidence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HandConfidence", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HandScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HandScale", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDominantHand(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDominantHand", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsPointerPoseValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsPointerPoseValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsSystemGestureInProgress(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsSystemGestureInProgress", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsTracked(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsTracked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PointerPose(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PointerPose", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRHand")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRHand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHand")]
impl AsRef<crate::GlobalNamespace::OVRMeshRenderer_IOVRMeshRendererDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::OVRMeshRenderer_IOVRMeshRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsMut<crate::GlobalNamespace::OVRMeshRenderer_IOVRMeshRendererDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRMeshRenderer_IOVRMeshRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsRef<crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsMut<crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::OVRMesh_IOVRMeshDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsRef<crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsMut<crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRSkeletonRenderer_IOVRSkeletonRendererDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsRef<crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand")]
impl AsMut<crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider>
for crate::GlobalNamespace::OVRHand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRSkeleton_IOVRSkeletonDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRHand+Hand")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHand_Hand {
    #[default]
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRHand+Hand")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRHand_Hand {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRHand/Hand";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRHand+Hand")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRHand_Hand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRHand+Hand")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRHand_Hand {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRHand+Hand")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRHand_Hand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRHand+Hand")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRHand_Hand {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVRHand+HandFinger")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHand_HandFinger {
    #[default]
    Index = 1i32,
    Max = 5i32,
    Middle = 2i32,
    Pinky = 4i32,
    Ring = 3i32,
    Thumb = 0i32,
}
#[cfg(feature = "OVRHand+HandFinger")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRHand_HandFinger {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRHand/HandFinger";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRHand+HandFinger")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRHand_HandFinger {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRHand+HandFinger")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRHand_HandFinger {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRHand+HandFinger")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRHand_HandFinger {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRHand+HandFinger")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRHand_HandFinger {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRHand_TrackingConfidence {
    #[default]
    High = 1065353216i32,
    Low = 0i32,
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRHand_TrackingConfidence {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRHand/TrackingConfidence";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRHand_TrackingConfidence {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRHand_TrackingConfidence {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRHand_TrackingConfidence {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRHand_TrackingConfidence {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
