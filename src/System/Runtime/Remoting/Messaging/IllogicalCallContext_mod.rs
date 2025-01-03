#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
#[repr(C)]
#[derive(Debug)]
pub struct IllogicalCallContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Datastore: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub m_HostContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::IllogicalCallContext =>
    "System.Runtime.Remoting.Messaging"."IllogicalCallContext"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::IllogicalCallContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::IllogicalCallContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
impl crate::System::Runtime::Remoting::Messaging::IllogicalCallContext {
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
        > = __cordl_object.invoke("CreateCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeNamedDataSlot(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeNamedDataSlot", (name))?;
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
    pub fn get_Datastore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        > = __cordl_object.invoke("get_Datastore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasUserData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasUserData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HostContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_HostContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HostContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HostContext", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IllogicalCallContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::IllogicalCallContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
