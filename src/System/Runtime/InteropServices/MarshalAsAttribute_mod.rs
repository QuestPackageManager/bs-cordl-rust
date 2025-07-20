#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MarshalAsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub MarshalCookie: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub MarshalType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub MarshalTypeRef: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub SafeArrayUserDefinedSubType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub utype: crate::System::Runtime::InteropServices::UnmanagedType,
    pub ArraySubType: crate::System::Runtime::InteropServices::UnmanagedType,
    pub SafeArraySubType: crate::System::Runtime::InteropServices::VarEnum,
    pub SizeConst: i32,
    pub IidParameterIndex: i32,
    pub SizeParamIndex: i16,
}
#[cfg(feature = "System+Runtime+InteropServices+MarshalAsAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::MarshalAsAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "MarshalAsAttribute";
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
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::MarshalAsAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::InteropServices::MarshalAsAttribute,
                >,
                0usize,
            >("Copy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::MarshalAsAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "Copy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::MarshalAsAttribute,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        unmanagedType: crate::System::Runtime::InteropServices::UnmanagedType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (unmanagedType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        unmanagedType: crate::System::Runtime::InteropServices::UnmanagedType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::MarshalAsAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Runtime::InteropServices::UnmanagedType),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::MarshalAsAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (unmanagedType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::InteropServices::UnmanagedType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::MarshalAsAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Runtime::InteropServices::UnmanagedType,
                0usize,
            >("get_Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::MarshalAsAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "get_Value", 0usize
                )
            });
        let __cordl_ret: crate::System::Runtime::InteropServices::UnmanagedType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
