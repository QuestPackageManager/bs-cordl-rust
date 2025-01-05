#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InstancedInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _instancedGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _rotationOffset: crate::UnityEngine::Vector3,
    pub _scaleOffset: crate::UnityEngine::Vector3,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InstancedInput =>
    "HoudiniEngineUnity"."HEU_InstancedInput"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InstancedInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InstancedInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl crate::HoudiniEngineUnity::HEU_InstancedInput {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InstancedInput>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InstancedInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InstancedInput>,
    >,
> for crate::HoudiniEngineUnity::HEU_InstancedInput {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InstancedInput>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InstancedInput")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InstancedInput>,
    >,
> for crate::HoudiniEngineUnity::HEU_InstancedInput {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InstancedInput>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
