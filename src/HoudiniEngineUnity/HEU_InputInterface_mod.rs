#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterface {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _priority: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputInterface =>
    "HoudiniEngineUnity"."HEU_InputInterface"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterface {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
impl crate::HoudiniEngineUnity::HEU_InputInterface {
    pub const DEFAULT_PRIORITY: i32 = 100i32;
    pub fn CreateInputNodeWithDataUpload(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        connectNodeID: i32,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        inputNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateInputNodeWithDataUpload",
                (session, connectNodeID, inputObject, inputNodeID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsThisInputObjectSupported", (inputObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (priority))?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterInterface(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterInterface", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Priority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Priority", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
