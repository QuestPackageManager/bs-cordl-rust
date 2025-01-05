#[cfg(feature = "OVRCompositionUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCompositionUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRCompositionUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRCompositionUtil => ""
    ."OVRCompositionUtil"
);
#[cfg(feature = "OVRCompositionUtil")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCompositionUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCompositionUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCompositionUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCompositionUtil")]
impl crate::GlobalNamespace::OVRCompositionUtil {
    pub fn BuildBoundaryMesh(
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
        topY: f32,
        bottomY: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildBoundaryMesh", (boundaryType, topY, bottomY))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaximumBoundaryDistance(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaximumBoundaryDistance", (camera, boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWorldPosition_Camera_Vector3_1(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        trackingSpacePosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWorldPosition", (camera, trackingSpacePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWorldPosition_Vector3_0(
        trackingSpacePosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWorldPosition", (trackingSpacePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SafeDestroy_ByRefMut1(
        obj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SafeDestroy", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeDestroy_GameObject0(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SafeDestroy", (obj))?;
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
}
#[cfg(feature = "OVRCompositionUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRCompositionUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
