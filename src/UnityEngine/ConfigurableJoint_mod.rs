#[cfg(feature = "UnityEngine+ConfigurableJoint")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurableJoint {
    __cordl_parent: crate::UnityEngine::Joint,
}
#[cfg(feature = "UnityEngine+ConfigurableJoint")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ConfigurableJoint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ConfigurableJoint";
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
#[cfg(feature = "UnityEngine+ConfigurableJoint")]
impl std::ops::Deref for crate::UnityEngine::ConfigurableJoint {
    type Target = crate::UnityEngine::Joint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ConfigurableJoint")]
impl std::ops::DerefMut for crate::UnityEngine::ConfigurableJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ConfigurableJoint")]
impl crate::UnityEngine::ConfigurableJoint {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_angularXDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_angularXDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularXDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularXDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularXLimitSpring(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimitSpring> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimitSpring = __cordl_object
            .invoke("get_angularXLimitSpring", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularXLimitSpring_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularXLimitSpring_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularXMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_angularXMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYLimit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimit = __cordl_object
            .invoke("get_angularYLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYLimit_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularYLimit_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_angularYMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYZDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_angularYZDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYZDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularYZDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYZLimitSpring(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimitSpring> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimitSpring = __cordl_object
            .invoke("get_angularYZLimitSpring", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularYZLimitSpring_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularYZLimitSpring_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularZLimit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimit = __cordl_object
            .invoke("get_angularZLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularZLimit_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_angularZLimit_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angularZMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_angularZMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_configuredInWorldSpace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_configuredInWorldSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highAngularXLimit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimit = __cordl_object
            .invoke("get_highAngularXLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highAngularXLimit_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_highAngularXLimit_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linearLimit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimit = __cordl_object
            .invoke("get_linearLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linearLimitSpring(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimitSpring> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimitSpring = __cordl_object
            .invoke("get_linearLimitSpring", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linearLimitSpring_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_linearLimitSpring_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linearLimit_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_linearLimit_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lowAngularXLimit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SoftJointLimit> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SoftJointLimit = __cordl_object
            .invoke("get_lowAngularXLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lowAngularXLimit_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_lowAngularXLimit_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionAngle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_projectionAngle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_projectionDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_projectionMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointProjectionMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointProjectionMode = __cordl_object
            .invoke("get_projectionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotationDriveMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RotationDriveMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RotationDriveMode = __cordl_object
            .invoke("get_rotationDriveMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secondaryAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_secondaryAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secondaryAxis_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_secondaryAxis_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_slerpDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_slerpDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_slerpDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_slerpDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_swapBodies(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_swapBodies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_targetAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetAngularVelocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetAngularVelocity_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_targetPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetPosition_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetPosition_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_targetRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetRotation_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetRotation_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_targetVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targetVelocity_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_targetVelocity_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_xDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_xDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_xMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_yDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_yDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_yMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zDrive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::JointDrive> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::JointDrive = __cordl_object
            .invoke("get_zDrive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zDrive_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_zDrive_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ConfigurableJointMotion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ConfigurableJointMotion = __cordl_object
            .invoke("get_zMotion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularXDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularXDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularXDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularXDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularXLimitSpring(
        &mut self,
        value: crate::UnityEngine::SoftJointLimitSpring,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularXLimitSpring", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularXLimitSpring_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularXLimitSpring_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularXMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularXMotion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYLimit(
        &mut self,
        value: crate::UnityEngine::SoftJointLimit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYLimit_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYLimit_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYMotion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYZDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYZDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYZDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYZDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYZLimitSpring(
        &mut self,
        value: crate::UnityEngine::SoftJointLimitSpring,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYZLimitSpring", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularYZLimitSpring_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularYZLimitSpring_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularZLimit(
        &mut self,
        value: crate::UnityEngine::SoftJointLimit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularZLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularZLimit_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularZLimit_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angularZMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_angularZMotion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_configuredInWorldSpace(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_configuredInWorldSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highAngularXLimit(
        &mut self,
        value: crate::UnityEngine::SoftJointLimit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highAngularXLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highAngularXLimit_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highAngularXLimit_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linearLimit(
        &mut self,
        value: crate::UnityEngine::SoftJointLimit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linearLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linearLimitSpring(
        &mut self,
        value: crate::UnityEngine::SoftJointLimitSpring,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linearLimitSpring", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linearLimitSpring_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimitSpring>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linearLimitSpring_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_linearLimit_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_linearLimit_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lowAngularXLimit(
        &mut self,
        value: crate::UnityEngine::SoftJointLimit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lowAngularXLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lowAngularXLimit_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SoftJointLimit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lowAngularXLimit_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionAngle(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_projectionAngle", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_projectionDistance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_projectionMode(
        &mut self,
        value: crate::UnityEngine::JointProjectionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_projectionMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotationDriveMode(
        &mut self,
        value: crate::UnityEngine::RotationDriveMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotationDriveMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_secondaryAxis(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_secondaryAxis", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_secondaryAxis_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_secondaryAxis_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_slerpDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_slerpDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_slerpDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_slerpDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_swapBodies(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_swapBodies", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetAngularVelocity(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetAngularVelocity_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetAngularVelocity_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetPosition(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetPosition_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetPosition_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetRotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetRotation_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetRotation_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetVelocity(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_targetVelocity_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetVelocity_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_xDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_xDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_xMotion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_yDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_yDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_yMotion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zDrive(
        &mut self,
        value: crate::UnityEngine::JointDrive,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_zDrive", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zDrive_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::JointDrive>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_zDrive_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zMotion(
        &mut self,
        value: crate::UnityEngine::ConfigurableJointMotion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_zMotion", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ConfigurableJoint")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ConfigurableJoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
