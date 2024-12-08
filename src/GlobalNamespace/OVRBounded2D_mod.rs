#[cfg(feature = "OVRBounded2D")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRBounded2D {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRBounded2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRBounded2D => ""
    ."OVRBounded2D"
);
#[cfg(feature = "OVRBounded2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRBounded2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRBounded2D")]
impl crate::GlobalNamespace::OVRBounded2D {
    pub fn ConvertRect(
        &mut self,
        openXrRect: crate::GlobalNamespace::OVRPlugin_Rectf,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConvertRect",
            (openXrRect),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_OVRBounded2D0(
        &mut self,
        other: crate::GlobalNamespace::OVRBounded2D,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__FromAnchor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRBounded2D> {
        let __cordl_ret: crate::GlobalNamespace::OVRBounded2D = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRBounded2D__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRBounded2D>.get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetBoundaryPoints(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetBoundaryPoints",
            (positions),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetBoundaryPointsCount(
        &mut self,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetBoundaryPointsCount",
            (count),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        anchor: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_BoundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BoundingBox",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
