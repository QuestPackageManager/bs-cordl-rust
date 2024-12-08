#[cfg(feature = "Unity+XR+Oculus+Boundary")]
#[repr(C)]
#[derive(Debug)]
pub struct Boundary {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Boundary => "Unity.XR.Oculus"
    ."Boundary"
);
#[cfg(feature = "Unity+XR+Oculus+Boundary")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Boundary {
    type Target = crate::System::Object;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary_BoundaryType {
    OuterBoundary = 0i32,
    PlayArea = 1i32,
}
#[cfg(feature = "Unity+XR+Oculus+Boundary+BoundaryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Boundary_BoundaryType =>
    "Unity.XR.Oculus"."Boundary/BoundaryType"
);
