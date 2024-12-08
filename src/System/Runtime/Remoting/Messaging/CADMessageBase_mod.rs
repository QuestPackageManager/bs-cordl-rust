#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMessageBase {
    __cordl_parent: crate::System::Object,
    pub _args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub _serializedArgs: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _propertyCount: i32,
    pub _callContext: *mut crate::System::Runtime::Remoting::Messaging::CADArgHolder,
    pub serializedMethod: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::CADMessageBase =>
    "System.Runtime.Remoting.Messaging"."CADMessageBase"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::CADMessageBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::CADMessageBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
impl crate::System::Runtime::Remoting::Messaging::CADMessageBase {
    pub fn UnmarshalArguments(
        &mut self,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("UnmarshalArguments", (arguments, args))?;
        Ok(__cordl_ret)
    }
    pub fn MarshalArgument(
        &mut self,
        arg: *mut crate::System::Object,
        args: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("MarshalArgument", (arg, args))?;
        Ok(__cordl_ret)
    }
    pub fn MarshalArguments(
        &mut self,
        arguments: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        args: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("MarshalArguments", (arguments, args))?;
        Ok(__cordl_ret)
    }
    pub fn GetLogicalCallContext(
        &mut self,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext = __cordl_object
            .invoke("GetLogicalCallContext", (args))?;
        Ok(__cordl_ret)
    }
    pub fn SaveLogicalCallContext(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        serializeList: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::ArrayList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveLogicalCallContext", (msg, serializeList))?;
        Ok(__cordl_ret)
    }
    pub fn UnmarshalArgument(
        &mut self,
        arg: *mut crate::System::Object,
        args: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("UnmarshalArgument", (arg, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn GetMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke("GetMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::CADMessageBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
