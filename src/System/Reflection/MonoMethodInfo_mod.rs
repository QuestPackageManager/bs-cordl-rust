#[cfg(feature = "System+Reflection+MonoMethodInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MonoMethodInfo {
    pub parent: *mut crate::System::Type,
    pub ret: *mut crate::System::Type,
    pub attrs: crate::System::Reflection::MethodAttributes,
    pub iattrs: crate::System::Reflection::MethodImplAttributes,
    pub callconv: crate::System::Reflection::CallingConventions,
}
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::MonoMethodInfo =>
    "System.Reflection"."MonoMethodInfo"
);
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::MonoMethodInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+MonoMethodInfo")]
impl crate::System::Reflection::MonoMethodInfo {
    pub fn GetAttributes(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MethodAttributes> {
        let __cordl_ret: crate::System::Reflection::MethodAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCallingConvention(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::CallingConventions> {
        let __cordl_ret: crate::System::Reflection::CallingConventions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCallingConvention", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeclaringType(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeclaringType", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodImplementationFlags(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MethodImplAttributes> {
        let __cordl_ret: crate::System::Reflection::MethodImplAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodImplementationFlags", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodInfo(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MonoMethodInfo> {
        let __cordl_ret: crate::System::Reflection::MonoMethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodInfo", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParametersInfo(
        handle: crate::System::IntPtr,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParametersInfo", (handle, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReturnParameterInfo(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeMethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReturnParameterInfo", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReturnType(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReturnType", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_method_attributes(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_method_attributes", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_method_info(
        handle: crate::System::IntPtr,
        info: quest_hook::libil2cpp::ByRefMut<crate::System::Reflection::MonoMethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_method_info", (handle, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parameter_info(
        handle: crate::System::IntPtr,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_parameter_info", (handle, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_retval_marshal(
        handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_retval_marshal", (handle))?;
        Ok(__cordl_ret.into())
    }
}
