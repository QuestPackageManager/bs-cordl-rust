#[cfg(feature = "PosePrediction")]
#[repr(C)]
#[derive(Debug)]
pub struct PosePrediction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PosePrediction")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PosePrediction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosePrediction";
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
#[cfg(feature = "PosePrediction")]
impl std::ops::Deref for crate::GlobalNamespace::PosePrediction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosePrediction")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosePrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosePrediction")]
impl crate::GlobalNamespace::PosePrediction {
    pub fn InterpolatePose(
        prev: crate::UnityEngine::Pose,
        curr: crate::UnityEngine::Pose,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolatePose", (prev, curr, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InterpolatePoseSerializable(
        a: crate::GlobalNamespace::PoseSerializable,
        b: crate::GlobalNamespace::PoseSerializable,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolatePoseSerializable", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PredictPose(
        prev: crate::UnityEngine::Pose,
        prevTime: i64,
        curr: crate::UnityEngine::Pose,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PredictPose", (prev, prevTime, curr, currTime, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn PredictPoseSerializable(
        prev: crate::GlobalNamespace::PoseSerializable,
        prevTime: i64,
        curr: crate::GlobalNamespace::PoseSerializable,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PredictPoseSerializable",
                (prev, prevTime, curr, currTime, _cordl_time),
            )?;
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
#[cfg(feature = "PosePrediction")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosePrediction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
