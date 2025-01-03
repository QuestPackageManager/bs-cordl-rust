#[cfg(feature = "VRUIControls+VRLaserPointer")]
#[repr(C)]
#[derive(Debug)]
pub struct VRLaserPointer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
}
#[cfg(feature = "VRUIControls+VRLaserPointer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::VRLaserPointer => "VRUIControls"
    ."VRLaserPointer"
);
#[cfg(feature = "VRUIControls+VRLaserPointer")]
impl std::ops::Deref for crate::VRUIControls::VRLaserPointer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRLaserPointer")]
impl std::ops::DerefMut for crate::VRUIControls::VRLaserPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRLaserPointer")]
impl crate::VRUIControls::VRLaserPointer {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetFadeDistance(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFadeDistance", (distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPosition(
        &mut self,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPosition", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalScale(
        &mut self,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalScale", (scale))?;
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
#[cfg(feature = "VRUIControls+VRLaserPointer")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRLaserPointer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
