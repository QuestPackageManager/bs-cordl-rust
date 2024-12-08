#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
#[repr(C)]
#[derive(Debug)]
pub struct CADMethodRef {
    __cordl_parent: crate::System::Object,
    pub ctor: bool,
    pub typeName: *mut crate::System::String,
    pub methodName: *mut crate::System::String,
    pub param_names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub generic_arg_names: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::CADMethodRef =>
    "System.Runtime.Remoting.Messaging"."CADMethodRef"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::CADMethodRef {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::CADMethodRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
impl crate::System::Runtime::Remoting::Messaging::CADMethodRef {
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
    pub fn Resolve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke("Resolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTypes(
        &mut self,
        typeArray: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetTypes", (typeArray))?;
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADMethodRef")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::CADMethodRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
