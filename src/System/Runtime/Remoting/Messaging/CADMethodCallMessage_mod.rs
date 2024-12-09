#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMethodCallMessage {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::CADMessageBase,
    pub _uri: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::CADMethodCallMessage =>
    "System.Runtime.Remoting.Messaging"."CADMethodCallMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage {
    type Target = crate::System::Runtime::Remoting::Messaging::CADMessageBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
impl crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage {
    pub fn GetArgs(
        &mut self,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
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
    pub fn New(
        callMsg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callMsg))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        callMsg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callMsg))?;
        Ok(__cordl_ret)
    }
    pub fn get_PropertiesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PropertiesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Uri", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodCallMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
