#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
#[repr(C)]
#[derive(Debug)]
pub struct ISynchronizeInvoke {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ISynchronizeInvoke =>
    "System.ComponentModel"."ISynchronizeInvoke"
);
#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
impl std::ops::Deref for crate::System::ComponentModel::ISynchronizeInvoke {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
impl std::ops::DerefMut for crate::System::ComponentModel::ISynchronizeInvoke {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
impl crate::System::ComponentModel::ISynchronizeInvoke {
    pub fn get_InvokeRequired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InvokeRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        method: *mut crate::System::Delegate,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (method, args))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+ISynchronizeInvoke")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ISynchronizeInvoke {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
