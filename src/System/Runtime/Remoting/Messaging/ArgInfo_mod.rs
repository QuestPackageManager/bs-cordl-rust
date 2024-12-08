#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ArgInfo {
    __cordl_parent: crate::System::Object,
    pub _paramMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _inoutArgCount: i32,
    pub _method: *mut crate::System::Reflection::MethodBase,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Messaging::ArgInfo =>
    "System.Runtime.Remoting.Messaging"."ArgInfo"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::ArgInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::ArgInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
impl crate::System::Runtime::Remoting::Messaging::ArgInfo {
    pub fn GetInOutArgs(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetInOutArgs", (args))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        method: *mut crate::System::Reflection::MethodBase,
        _cordl_type: crate::System::Runtime::Remoting::Messaging::ArgInfoType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, _cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Reflection::MethodBase,
        _cordl_type: crate::System::Runtime::Remoting::Messaging::ArgInfoType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, _cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ArgInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
