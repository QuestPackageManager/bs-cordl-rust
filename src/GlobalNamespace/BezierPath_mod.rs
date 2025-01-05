#[cfg(feature = "BezierPath")]
#[repr(C)]
#[derive(Debug)]
pub struct BezierPath {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub bezierPathWasModifiedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub _controlMode: crate::GlobalNamespace::BezierPath_ControlMode,
    pub _perAnchorNormalsAngle: quest_hook::libil2cpp::Gc<f32>,
    pub _neighbourDistances: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
}
#[cfg(feature = "BezierPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BezierPath => ""."BezierPath"
);
#[cfg(feature = "BezierPath")]
impl std::ops::Deref for crate::GlobalNamespace::BezierPath {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BezierPath")]
impl std::ops::DerefMut for crate::GlobalNamespace::BezierPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BezierPath")]
impl crate::GlobalNamespace::BezierPath {
    pub const kAutoControlLength: f32 = 0.3f32;
    #[cfg(feature = "BezierPath+ControlMode")]
    pub type ControlMode = crate::GlobalNamespace::BezierPath_ControlMode;
    pub fn AddSegmentToEnd(
        &mut self,
        anchorPos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSegmentToEnd", (anchorPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoSetAllAffectedControlPoints(
        &mut self,
        updatedAnchorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoSetAllAffectedControlPoints", (updatedAnchorIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoSetAllControlPoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoSetAllControlPoints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoSetAnchorControlPoints(
        &mut self,
        anchorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoSetAnchorControlPoints", (anchorIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AutoSetStartAndEndControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoSetStartAndEndControls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAnchorNormalAngle(
        &mut self,
        anchorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetAnchorNormalAngle", (anchorIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPoint(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetPoint", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPointsInSegment_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        segmentIndex: i32,
        p0: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p2: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        p3: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPointsInSegment", (segmentIndex, p0, p1, p2, p3))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPointsInSegment_i32_ByRefMut0(
        &mut self,
        segmentIndex: i32,
        points: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPointsInSegment", (segmentIndex, points))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoopIndex(&mut self, i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LoopIndex", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        centre: crate::UnityEngine::Vector3,
        initTwoSegments: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (centre, initTwoSegments))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyPathModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyPathModified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAnchorNormalAngle(
        &mut self,
        anchorIndex: i32,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAnchorNormalAngle", (anchorIndex, angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPoint(
        &mut self,
        i: i32,
        localPosition: crate::UnityEngine::Vector3,
        suppressPathModified: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPoint", (i, localPosition, suppressPathModified))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateByAnchorPoints(
        &mut self,
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateByAnchorPoints", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateControlPoints(
        &mut self,
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateControlPoints", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        centre: crate::UnityEngine::Vector3,
        initTwoSegments: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (centre, initTwoSegments))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_bezierPathWasModifiedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_bezierPathWasModifiedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_Item", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anchorPointsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_anchorPointsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlPointMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BezierPath_ControlMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BezierPath_ControlMode = __cordl_object
            .invoke("get_controlPointMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pointsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_segmentsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_segmentsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_bezierPathWasModifiedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_bezierPathWasModifiedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controlPointMode(
        &mut self,
        value: crate::GlobalNamespace::BezierPath_ControlMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPointMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BezierPath")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BezierPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BezierPath+ControlMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BezierPath_ControlMode {
    #[default]
    Aligned = 0i32,
    Automatic = 3i32,
    Free = 2i32,
    Mirrored = 1i32,
}
#[cfg(feature = "BezierPath+ControlMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BezierPath_ControlMode => ""
    ."BezierPath/ControlMode"
);
