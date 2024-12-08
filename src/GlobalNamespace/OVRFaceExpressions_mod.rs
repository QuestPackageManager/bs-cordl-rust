#[cfg(feature = "OVRFaceExpressions+FaceExpression")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRFaceExpressions_FaceExpression {
    BrowLowererL = 0i32,
    BrowLowererR = 1i32,
    CheekPuffL = 2i32,
    CheekPuffR = 3i32,
    CheekRaiserL = 4i32,
    CheekRaiserR = 5i32,
    CheekSuckL = 6i32,
    CheekSuckR = 7i32,
    ChinRaiserB = 8i32,
    ChinRaiserT = 9i32,
    DimplerL = 10i32,
    DimplerR = 11i32,
    EyesClosedL = 12i32,
    EyesClosedR = 13i32,
    EyesLookDownL = 14i32,
    EyesLookDownR = 15i32,
    EyesLookLeftL = 16i32,
    EyesLookLeftR = 17i32,
    EyesLookRightL = 18i32,
    EyesLookRightR = 19i32,
    EyesLookUpL = 20i32,
    EyesLookUpR = 21i32,
    InnerBrowRaiserL = 22i32,
    InnerBrowRaiserR = 23i32,
    Invalid = -1i32,
    JawDrop = 24i32,
    JawSidewaysLeft = 25i32,
    JawSidewaysRight = 26i32,
    JawThrust = 27i32,
    LidTightenerL = 28i32,
    LidTightenerR = 29i32,
    LipCornerDepressorL = 30i32,
    LipCornerDepressorR = 31i32,
    LipCornerPullerL = 32i32,
    LipCornerPullerR = 33i32,
    LipFunnelerLB = 34i32,
    LipFunnelerLT = 35i32,
    LipFunnelerRB = 36i32,
    LipFunnelerRT = 37i32,
    LipPressorL = 38i32,
    LipPressorR = 39i32,
    LipPuckerL = 40i32,
    LipPuckerR = 41i32,
    LipStretcherL = 42i32,
    LipStretcherR = 43i32,
    LipSuckLB = 44i32,
    LipSuckLT = 45i32,
    LipSuckRB = 46i32,
    LipSuckRT = 47i32,
    LipTightenerL = 48i32,
    LipTightenerR = 49i32,
    LipsToward = 50i32,
    LowerLipDepressorL = 51i32,
    LowerLipDepressorR = 52i32,
    Max = 63i32,
    MouthLeft = 53i32,
    MouthRight = 54i32,
    NoseWrinklerL = 55i32,
    NoseWrinklerR = 56i32,
    OuterBrowRaiserL = 57i32,
    OuterBrowRaiserR = 58i32,
    UpperLidRaiserL = 59i32,
    UpperLidRaiserR = 60i32,
    UpperLipRaiserL = 61i32,
    UpperLipRaiserR = 62i32,
}
#[cfg(feature = "OVRFaceExpressions+FaceExpression")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRFaceExpressions_FaceExpression => ""
    ."OVRFaceExpressions/FaceExpression"
);
#[cfg(feature = "OVRFaceExpressions+FaceExpressionsEnumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRFaceExpressions_FaceExpressionsEnumerator {
    pub _faceExpressions: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _index: i32,
    pub _count: i32,
}
#[cfg(feature = "OVRFaceExpressions+FaceExpressionsEnumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator => ""
    ."OVRFaceExpressions/FaceExpressionsEnumerator"
);
#[cfg(feature = "OVRFaceExpressions+FaceExpressionsEnumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRFaceExpressions+FaceExpressionsEnumerator")]
impl crate::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator {
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRFaceExpressions+FaceRegionConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRFaceExpressions_FaceRegionConfidence {
    Lower = 0i32,
    Max = 2i32,
    Upper = 1i32,
}
#[cfg(feature = "OVRFaceExpressions+FaceRegionConfidence")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRFaceExpressions_FaceRegionConfidence => ""
    ."OVRFaceExpressions/FaceRegionConfidence"
);
#[cfg(feature = "OVRFaceExpressions")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRFaceExpressions {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _ValidExpressions_k__BackingField: bool,
    pub _EyeFollowingBlendshapesValid_k__BackingField: bool,
    pub _currentFaceState: crate::GlobalNamespace::OVRPlugin_FaceState,
    pub _onPermissionGranted: *mut crate::System::Action_1<*mut crate::System::String>,
}
#[cfg(feature = "OVRFaceExpressions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRFaceExpressions => ""."OVRFaceExpressions"
);
#[cfg(feature = "OVRFaceExpressions")]
impl std::ops::Deref for OVRFaceExpressions {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFaceExpressions")]
impl std::ops::DerefMut for OVRFaceExpressions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFaceExpressions")]
impl OVRFaceExpressions {
    #[cfg(feature = "OVRFaceExpressions+FaceRegionConfidence")]
    pub type FaceRegionConfidence = crate::GlobalNamespace::OVRFaceExpressions_FaceRegionConfidence;
    #[cfg(feature = "OVRFaceExpressions+FaceExpressionsEnumerator")]
    pub type FaceExpressionsEnumerator = crate::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator;
    #[cfg(feature = "OVRFaceExpressions+FaceExpression")]
    pub type FaceExpression = crate::GlobalNamespace::OVRFaceExpressions_FaceExpression;
    #[cfg(feature = "OVRFaceExpressions+WeightProvider")]
    type WeightProvider = crate::GlobalNamespace::OVRFaceExpressions_WeightProvider;
    pub fn GetWeight(
        &mut self,
        expression: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetWeight", (expression))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
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
    pub fn get_FaceTrackingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_FaceTrackingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetWeightConfidence(
        &mut self,
        region: crate::GlobalNamespace::OVRFaceExpressions_FaceRegionConfidence,
        weightConfidence: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetWeightConfidence", (region, weightConfidence))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRFaceExpressions_FaceExpressionsEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, startIndex))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_System_Single__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<f32> = __cordl_object
            .invoke(
                "System.Collections.Generic.IEnumerable<System.Single>.GetEnumerator",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidExpressions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ValidExpressions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EyeFollowingBlendshapesValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_EyeFollowingBlendshapesValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_EyeFollowingBlendshapesValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EyeFollowingBlendshapesValid", (value))?;
        Ok(__cordl_ret)
    }
    pub fn StartFaceTracking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("StartFaceTracking", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPermissionGranted(
        &mut self,
        permissionId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPermissionGranted", (permissionId))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetFaceExpressionWeight(
        &mut self,
        expression: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
        weight: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetFaceExpressionWeight", (expression, weight))?;
        Ok(__cordl_ret)
    }
    pub fn set_ValidExpressions(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidExpressions", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        expression: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Item", (expression))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValidity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckValidity", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("ToArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRFaceExpressions")]
impl quest_hook::libil2cpp::ObjectType for OVRFaceExpressions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRFaceExpressions_WeightProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRFaceExpressions_WeightProvider => ""
    ."OVRFaceExpressions/WeightProvider"
);
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
impl std::ops::Deref for crate::GlobalNamespace::OVRFaceExpressions_WeightProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRFaceExpressions_WeightProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
impl crate::GlobalNamespace::OVRFaceExpressions_WeightProvider {
    pub fn GetWeight(
        &mut self,
        expression: crate::GlobalNamespace::OVRFaceExpressions_FaceExpression,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetWeight", (expression))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRFaceExpressions+WeightProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRFaceExpressions_WeightProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
