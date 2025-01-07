#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMethodReturnMessage {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::CADMessageBase,
    pub _returnValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _exception: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::CADArgHolder,
    >,
    pub _sig: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodReturnMessage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Messaging";
    const CLASS_NAME: &'static str = "CADMethodReturnMessage";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn Create(
        callMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", (callMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgs(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
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
        > = __cordl_object.invoke("GetArgs", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("GetArguments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetException(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("GetException", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReturnValue(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetReturnValue", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        retMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (retMsg))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        retMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (retMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertiesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PropertiesCount", ())?;
        Ok(__cordl_ret.into())
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
