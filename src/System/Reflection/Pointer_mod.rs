#[cfg(feature = "System+Reflection+Pointer")]
#[repr(C)]
#[derive(Debug)]
pub struct Pointer {
    __cordl_parent: crate::System::Object,
    pub _ptr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _ptrType: *mut crate::System::Type,
}
#[cfg(feature = "System+Reflection+Pointer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Pointer =>
    "System.Reflection"."Pointer"
);
#[cfg(feature = "System+Reflection+Pointer")]
impl std::ops::Deref for crate::System::Reflection::Pointer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Pointer")]
impl std::ops::DerefMut for crate::System::Reflection::Pointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Pointer")]
impl crate::System::Reflection::Pointer {
    pub fn New(
        ptr: *mut quest_hook::libil2cpp::Il2CppObject,
        ptrType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ptr, ptrType))?;
        Ok(__cordl_object)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ptr: *mut quest_hook::libil2cpp::Il2CppObject,
        ptrType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ptr, ptrType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+Pointer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::Pointer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}