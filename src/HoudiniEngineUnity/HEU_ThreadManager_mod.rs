#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ThreadManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tasks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ThreadedTask,
        >,
    >,
    pub _pendingAdd: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ThreadedTask,
        >,
    >,
    pub _pendingRemove: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ThreadedTask,
        >,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ThreadManager =>
    "HoudiniEngineUnity"."HEU_ThreadManager"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ThreadManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ThreadManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
impl crate::HoudiniEngineUnity::HEU_ThreadManager {
    pub fn AddTask(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ThreadedTask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Register(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveTask(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ThreadedTask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unregister(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unregister", ())?;
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
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ThreadManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadManager,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadManager")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ThreadManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
