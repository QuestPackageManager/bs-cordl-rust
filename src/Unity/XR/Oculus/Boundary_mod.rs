#[cfg(feature = "Unity+XR+Oculus+Boundary")]
#[repr(C)]
#[derive(Debug)]
pub struct Boundary {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Boundary => "Unity.XR.Oculus"
    ."Boundary"
);
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Boundary {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Boundary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
impl crate::Unity::XR::Oculus::Boundary {
    #[cfg(feature = "Unity+XR+Oculus+Boundary+BoundaryType")]
    pub type BoundaryType = crate::Unity::XR::Oculus::Boundary_BoundaryType;
    pub fn GetBoundaryConfigured() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryConfigured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryDimensions(
        boundaryType: crate::Unity::XR::Oculus::Boundary_BoundaryType,
        dimensions: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryDimensions", (boundaryType, dimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryVisible() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoundaryVisible(
        boundaryVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBoundaryVisible", (boundaryVisible))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Boundary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Boundary+BoundaryType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Boundary_BoundaryType {
    #[default]
    OuterBoundary = 0i32,
    PlayArea = 1i32,
}
#[cfg(feature = "Unity+XR+Oculus+Boundary+BoundaryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Boundary_BoundaryType =>
    "Unity.XR.Oculus"."Boundary/BoundaryType"
);
