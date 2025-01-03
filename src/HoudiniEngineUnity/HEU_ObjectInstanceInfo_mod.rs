#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ObjectInstanceInfo {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _instancedInputs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InstancedInput,
        >,
    >,
    pub _partTarget: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    pub _instancedObjectNodeID: i32,
    pub _instancedObjectPath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _instances: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::UnityEngine::GameObject>,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ObjectInstanceInfo =>
    "HoudiniEngineUnity"."HEU_ObjectInstanceInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        >,
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    >,
> for crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ObjectInstanceInfo")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    >,
> for crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
