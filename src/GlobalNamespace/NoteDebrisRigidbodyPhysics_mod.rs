#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisRigidbodyPhysics {
    __cordl_parent: crate::GlobalNamespace::NoteDebrisPhysics,
    pub _rigidbody: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
    pub _simplePhysics: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebrisSimplePhysics,
    >,
    pub _firstUpdate: bool,
}
#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebrisRigidbodyPhysics =>
    ""."NoteDebrisRigidbodyPhysics"
);
#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisRigidbodyPhysics {
    type Target = crate::GlobalNamespace::NoteDebrisPhysics;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebrisRigidbodyPhysics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
impl crate::GlobalNamespace::NoteDebrisRigidbodyPhysics {
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
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
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
}
#[cfg(feature = "NoteDebrisRigidbodyPhysics")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteDebrisRigidbodyPhysics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
