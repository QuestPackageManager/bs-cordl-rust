#[cfg(feature = "OVRBoneCapsule")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRBoneCapsule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _BoneIndex_k__BackingField: i16,
    pub _CapsuleRigidbody_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rigidbody,
    >,
    pub _CapsuleCollider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CapsuleCollider,
    >,
}
#[cfg(feature = "OVRBoneCapsule")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRBoneCapsule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRBoneCapsule";
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
#[cfg(feature = "OVRBoneCapsule")]
impl std::ops::Deref for crate::GlobalNamespace::OVRBoneCapsule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBoneCapsule")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRBoneCapsule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRBoneCapsule")]
impl crate::GlobalNamespace::OVRBoneCapsule {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i16_Rigidbody_CapsuleCollider1(
        boneIndex: i16,
        capsuleRigidBody: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
        capsuleCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::CapsuleCollider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boneIndex, capsuleRigidBody, capsuleCollider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i16_Rigidbody_CapsuleCollider1(
        &mut self,
        boneIndex: i16,
        capsuleRigidBody: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
        capsuleCollider: quest_hook::libil2cpp::Gc<crate::UnityEngine::CapsuleCollider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (boneIndex, capsuleRigidBody, capsuleCollider))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BoneIndex(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("get_BoneIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CapsuleCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::CapsuleCollider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::CapsuleCollider,
        > = __cordl_object.invoke("get_CapsuleCollider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CapsuleRigidbody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody> = __cordl_object
            .invoke("get_CapsuleRigidbody", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BoneIndex(
        &mut self,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BoneIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CapsuleCollider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::CapsuleCollider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CapsuleCollider", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CapsuleRigidbody(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rigidbody>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CapsuleRigidbody", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRBoneCapsule")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRBoneCapsule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
