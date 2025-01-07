#[cfg(feature = "NoteDebrisSimplePhysics")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisSimplePhysics {
    __cordl_parent: crate::GlobalNamespace::NoteDebrisPhysics,
    pub _currentLinearVelocity: crate::UnityEngine::Vector3,
    pub _currentAngularVelocityDegrees: crate::UnityEngine::Vector3,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _gravity: crate::UnityEngine::Vector3,
    pub _firstUpdate: bool,
    pub _position: crate::UnityEngine::Vector3,
    pub _rotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "NoteDebrisSimplePhysics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteDebrisSimplePhysics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteDebrisSimplePhysics";
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
    pub fn Init(
        &mut self,
        linearVelocity: crate::UnityEngine::Vector3,
        angularVelocity: crate::UnityEngine::Vector3,
        forceOnlySimplePhysics: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (linearVelocity, angularVelocity, forceOnlySimplePhysics))?;
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
    pub fn get_currentAngularVelocityDegrees(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_currentAngularVelocityDegrees", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentLinearVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_currentLinearVelocity", ())?;
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
