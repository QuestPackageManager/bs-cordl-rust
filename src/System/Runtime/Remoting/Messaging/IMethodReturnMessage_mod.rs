#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct IMethodReturnMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::IMethodReturnMessage =>
    "System.Runtime.Remoting.Messaging"."IMethodReturnMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutArgs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("get_OutArgs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ReturnValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
> for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
> for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    >,
> for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IMethodReturnMessage")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    >,
> for crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
