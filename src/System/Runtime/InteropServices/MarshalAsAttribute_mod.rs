#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MarshalAsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub MarshalCookie: *mut quest_hook::libil2cpp::Il2CppString,
    pub MarshalType: *mut quest_hook::libil2cpp::Il2CppString,
    pub MarshalTypeRef: *mut crate::System::Type,
    pub SafeArrayUserDefinedSubType: *mut crate::System::Type,
    pub utype: crate::System::Runtime::InteropServices::UnmanagedType,
    pub ArraySubType: crate::System::Runtime::InteropServices::UnmanagedType,
    pub SafeArraySubType: crate::System::Runtime::InteropServices::VarEnum,
    pub SizeConst: i32,
    pub IidParameterIndex: i32,
    pub SizeParamIndex: i16,
}
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::MarshalAsAttribute =>
    "System.Runtime.InteropServices"."MarshalAsAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::MarshalAsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::MarshalAsAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
impl crate::System::Runtime::InteropServices::MarshalAsAttribute {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::InteropServices::MarshalAsAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::InteropServices::MarshalAsAttribute = __cordl_object
            .invoke("Copy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        unmanagedType: crate::System::Runtime::InteropServices::UnmanagedType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unmanagedType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        unmanagedType: crate::System::Runtime::InteropServices::UnmanagedType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (unmanagedType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::UnmanagedType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::InteropServices::UnmanagedType = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::MarshalAsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
