#[cfg(feature = "NoteDebrisSimplePhysics")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisSimplePhysics {
    __cordl_parent: crate::GlobalNamespace::NoteDebrisPhysics,
    pub _currentForce: crate::UnityEngine::Vector3,
    pub _currentTorque: crate::UnityEngine::Vector3,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _gravity: crate::UnityEngine::Vector3,
    pub _firstUpdate: bool,
}
#[cfg(feature = "NoteDebrisSimplePhysics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebrisSimplePhysics => ""
    ."NoteDebrisSimplePhysics"
);
#[cfg(feature = "NoteDebrisSimplePhysics")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisSimplePhysics {
    type Target = crate::GlobalNamespace::NoteDebrisPhysics;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSimplePhysics")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebrisSimplePhysics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisSimplePhysics")]
impl crate::GlobalNamespace::NoteDebrisSimplePhysics {
    pub fn AddVelocity(
        &mut self,
        force: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddVelocity", (force))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        force: crate::UnityEngine::Vector3,
        torque: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (force, torque))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteDebrisSimplePhysics")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteDebrisSimplePhysics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
