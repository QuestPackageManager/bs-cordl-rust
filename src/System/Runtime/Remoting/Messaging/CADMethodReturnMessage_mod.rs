#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMethodReturnMessage {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::CADMessageBase,
    pub _returnValue: *mut crate::System::Object,
    pub _exception: *mut crate::System::Runtime::Remoting::Messaging::CADArgHolder,
    pub _sig: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::CADMethodReturnMessage =>
    "System.Runtime.Remoting.Messaging"."CADMethodReturnMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage {
    type Target = crate::System::Runtime::Remoting::Messaging::CADMessageBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
impl crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage {
    pub fn GetReturnValue(
        &mut self,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetReturnValue", (args))?;
        Ok(__cordl_ret)
    }
    pub fn get_PropertiesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PropertiesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        retMsg: *mut crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (retMsg))?;
        Ok(__cordl_ret)
    }
    pub fn GetArgs(
        &mut self,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetArgs", (args))?;
        Ok(__cordl_ret)
    }
    pub fn GetArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("GetArguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetException(
        &mut self,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("GetException", (args))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        retMsg: *mut crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (retMsg))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
