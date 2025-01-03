#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMessageBase")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMessageBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _args: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetLogicalCallContext(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        > = __cordl_object.invoke("GetLogicalCallContext", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = __cordl_object.invoke("GetMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignature(
        methodBase: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        load: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignature", (methodBase, load))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPossibleToIgnoreMarshal(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPossibleToIgnoreMarshal", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarshalArgument(
        &mut self,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        args: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("MarshalArgument", (arg, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarshalArguments(
        &mut self,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        args: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("MarshalArguments", (arguments, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarshalProperties(
        dict: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        args: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarshalProperties", (dict, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg))?;
        Ok(__cordl_object.into())
    }
    pub fn SaveLogicalCallContext(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
        serializeList: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::ArrayList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveLogicalCallContext", (msg, serializeList))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmarshalArgument(
        &mut self,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("UnmarshalArgument", (arg, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmarshalArguments(
        &mut self,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("UnmarshalArguments", (arguments, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmarshalProperties(
        dict: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        count: i32,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnmarshalProperties", (dict, count, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg))?;
        Ok(__cordl_ret.into())
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
