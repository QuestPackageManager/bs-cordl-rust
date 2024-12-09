#[cfg(feature = "OVRBoundary")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRBoundary {
    __cordl_parent: crate::System::Object,
    pub cachedGeometryList: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "OVRBoundary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBoundary => ""."OVRBoundary"
);
#[cfg(feature = "OVRBoundary")]
impl std::ops::Deref for crate::GlobalNamespace::OVRBoundary {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBoundary")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRBoundary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBoundary")]
impl crate::GlobalNamespace::OVRBoundary {
    #[cfg(feature = "OVRBoundary+BoundaryTestResult")]
    pub type BoundaryTestResult = crate::GlobalNamespace::OVRBoundary_BoundaryTestResult;
    #[cfg(feature = "OVRBoundary+BoundaryType")]
    pub type BoundaryType = crate::GlobalNamespace::OVRBoundary_BoundaryType;
    #[cfg(feature = "OVRBoundary+Node")]
    pub type Node = crate::GlobalNamespace::OVRBoundary_Node;
    pub fn GetConfigured(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetConfigured", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDimensions(
        &mut self,
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetDimensions", (boundaryType))?;
        Ok(__cordl_ret)
    }
    pub fn GetGeometry(
        &mut self,
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector3,
        > = __cordl_object.invoke("GetGeometry", (boundaryType))?;
        Ok(__cordl_ret)
    }
    pub fn GetVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetVisible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisible", (value))?;
        Ok(__cordl_ret)
    }
    pub fn TestNode(
        &mut self,
        node: crate::GlobalNamespace::OVRBoundary_Node,
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRBoundary_BoundaryTestResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRBoundary_BoundaryTestResult = __cordl_object
            .invoke("TestNode", (node, boundaryType))?;
        Ok(__cordl_ret)
    }
    pub fn TestPoint(
        &mut self,
        point: crate::UnityEngine::Vector3,
        boundaryType: crate::GlobalNamespace::OVRBoundary_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRBoundary_BoundaryTestResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRBoundary_BoundaryTestResult = __cordl_object
            .invoke("TestPoint", (point, boundaryType))?;
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
}
#[cfg(feature = "OVRBoundary")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRBoundary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRBoundary+BoundaryTestResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRBoundary_BoundaryTestResult {
    pub IsTriggering: bool,
    pub ClosestDistance: f32,
    pub ClosestPoint: crate::UnityEngine::Vector3,
    pub ClosestPointNormal: crate::UnityEngine::Vector3,
}
#[cfg(feature = "OVRBoundary+BoundaryTestResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBoundary_BoundaryTestResult
    => ""."OVRBoundary/BoundaryTestResult"
);
#[cfg(feature = "OVRBoundary+BoundaryTestResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRBoundary_BoundaryTestResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRBoundary+BoundaryTestResult")]
impl crate::GlobalNamespace::OVRBoundary_BoundaryTestResult {}
#[cfg(feature = "OVRBoundary+BoundaryType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRBoundary_BoundaryType {
    OuterBoundary = 1i32,
    PlayArea = 256i32,
}
#[cfg(feature = "OVRBoundary+BoundaryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBoundary_BoundaryType => ""
    ."OVRBoundary/BoundaryType"
);
#[cfg(feature = "OVRBoundary+Node")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRBoundary_Node {
    HandLeft = 3i32,
    HandRight = 4i32,
    Head = 9i32,
}
#[cfg(feature = "OVRBoundary+Node")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBoundary_Node => ""
    ."OVRBoundary/Node"
);
