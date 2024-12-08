#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct UnmanagedFunctionPointerAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
}
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::UnmanagedFunctionPointerAttribute =>
    "System.Runtime.InteropServices"."UnmanagedFunctionPointerAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::UnmanagedFunctionPointerAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::UnmanagedFunctionPointerAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
impl crate::System::Runtime::InteropServices::UnmanagedFunctionPointerAttribute {
    pub fn _ctor(
        &mut self,
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callingConvention))?;
        Ok(__cordl_ret)
    }
    pub fn get_CallingConvention(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::CallingConvention,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::InteropServices::CallingConvention = __cordl_object
            .invoke("get_CallingConvention", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callingConvention))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedFunctionPointerAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::UnmanagedFunctionPointerAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
