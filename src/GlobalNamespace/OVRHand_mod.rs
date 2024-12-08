#[cfg(feature = "OVRHand+Hand")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRHand_Hand {
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRHand+Hand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHand_Hand => ""
    ."OVRHand/Hand"
);
#[cfg(feature = "OVRHand+HandFinger")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRHand_HandFinger {
    Index = 1i32,
    Max = 5i32,
    Middle = 2i32,
    Pinky = 4i32,
    Ring = 3i32,
    Thumb = 0i32,
}
#[cfg(feature = "OVRHand+HandFinger")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHand_HandFinger => ""
    ."OVRHand/HandFinger"
);
#[cfg(feature = "OVRHand")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHand {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub HandType: crate::GlobalNamespace::OVRHand_Hand,
    pub _pointerPoseRoot: *mut crate::UnityEngine::Transform,
    pub m_showState: crate::GlobalNamespace::OVRInput_InputDeviceShowState,
    pub _pointerPoseGO: *mut crate::UnityEngine::GameObject,
    pub _handState: crate::GlobalNamespace::OVRPlugin_HandState,
    pub _IsDataValid_k__BackingField: bool,
    pub _IsDataHighConfidence_k__BackingField: bool,
    pub _IsTracked_k__BackingField: bool,
    pub _IsSystemGestureInProgress_k__BackingField: bool,
    pub _IsPointerPoseValid_k__BackingField: bool,
    pub _PointerPose_k__BackingField: *mut crate::UnityEngine::Transform,
    pub _HandScale_k__BackingField: f32,
    pub _HandConfidence_k__BackingField: crate::GlobalNamespace::OVRHand_TrackingConfidence,
    pub _IsDominantHand_k__BackingField: bool,
}
#[cfg(feature = "OVRHand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRHand => ""."OVRHand"
);
#[cfg(feature = "OVRHand")]
impl std::ops::Deref for OVRHand {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHand")]
impl std::ops::DerefMut for OVRHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHand")]
impl OVRHand {
    #[cfg(feature = "OVRHand+Hand")]
    pub type Hand = crate::GlobalNamespace::OVRHand_Hand;
    #[cfg(feature = "OVRHand+HandFinger")]
    pub type HandFinger = crate::GlobalNamespace::OVRHand_HandFinger;
    #[cfg(feature = "OVRHand+TrackingConfidence")]
    pub type TrackingConfidence = crate::GlobalNamespace::OVRHand_TrackingConfidence;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_IsDataHighConfidence(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataHighConfidence", ())?;
        Ok(__cordl_ret)
    }
    pub fn OVRSkeleton_IOVRSkeletonDataProvider_get_enabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRSkeleton.IOVRSkeletonDataProvider.get_enabled", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetFingerIsPinching(
        &mut self,
        finger: crate::GlobalNamespace::OVRHand_HandFinger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetFingerIsPinching", (finger))?;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_HandScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_HandScale", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_IsPointerPoseValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPointerPoseValid", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_PointerPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_PointerPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDataValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDataValid", ())?;
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
    pub fn get_IsTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTracked", ())?;
        Ok(__cordl_ret)
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
    pub fn set_PointerPose(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PointerPose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OVRMesh_IOVRMeshDataProvider_GetMeshType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMesh_MeshType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMesh_MeshType = __cordl_object
            .invoke("OVRMesh.IOVRMeshDataProvider.GetMeshType", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_IsDominantHand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDominantHand", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSystemGestureInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsSystemGestureInProgress", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRHand")]
impl quest_hook::libil2cpp::ObjectType for OVRHand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRHand_TrackingConfidence {
    High = 1065353216i32,
    Low = 0i32,
}
#[cfg(feature = "OVRHand+TrackingConfidence")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHand_TrackingConfidence =>
    ""."OVRHand/TrackingConfidence"
);
