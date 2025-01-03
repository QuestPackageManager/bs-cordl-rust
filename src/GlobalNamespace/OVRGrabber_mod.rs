#[cfg(feature = "OVRGrabber")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGrabber {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub grabBegin: f32,
    pub grabEnd: f32,
    pub m_parentHeldObject: bool,
    pub m_moveHandPosition: bool,
    pub m_gripTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub m_grabVolumes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Collider>,
    >,
    pub m_controller: crate::GlobalNamespace::OVRInput_Controller,
    pub m_parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub m_player: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub m_grabVolumeEnabled: bool,
    pub m_lastPos: crate::UnityEngine::Vector3,
    pub m_lastRot: crate::UnityEngine::Quaternion,
    pub m_anchorOffsetRotation: crate::UnityEngine::Quaternion,
    pub m_anchorOffsetPosition: crate::UnityEngine::Vector3,
    pub m_prevFlex: f32,
    pub m_grabbedObj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGrabbable>,
    pub m_grabbedObjectPosOff: crate::UnityEngine::Vector3,
    pub m_grabbedObjectRotOff: crate::UnityEngine::Quaternion,
    pub m_grabCandidates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::OVRGrabbable,
            i32,
        >,
    >,
    pub m_operatingWithoutOVRCameraRig: bool,
}
#[cfg(feature = "OVRGrabber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGrabber => ""."OVRGrabber"
);
#[cfg(feature = "OVRGrabber")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGrabber {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGrabber")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGrabber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGrabber")]
impl crate::GlobalNamespace::OVRGrabber {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForGrabOrRelease(
        &mut self,
        prevFlex: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckForGrabOrRelease", (prevFlex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceRelease(
        &mut self,
        grabbable: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGrabbable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceRelease", (grabbable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabBegin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrabBegin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrabEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabVolumeEnable(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrabVolumeEnable", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabbableRelease(
        &mut self,
        linearVelocity: crate::UnityEngine::Vector3,
        angularVelocity: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GrabbableRelease", (linearVelocity, angularVelocity))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveGrabbedObject(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
        forceTeleport: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveGrabbedObject", (pos, rot, forceTeleport))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OffhandGrabbed(
        &mut self,
        grabbable: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGrabbable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OffhandGrabbed", (grabbable))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTriggerEnter(
        &mut self,
        otherCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTriggerEnter", (otherCollider))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTriggerExit(
        &mut self,
        otherCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTriggerExit", (otherCollider))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdatedAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdatedAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerIgnoreCollision(
        &mut self,
        grabbable: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        ignore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerIgnoreCollision", (grabbable, ignore))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__23_0(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__23_0", (r))?;
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
    pub fn get_grabbedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGrabbable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRGrabbable,
        > = __cordl_object.invoke("get_grabbedObject", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRGrabber")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRGrabber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
