#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DllImportAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub EntryPoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub CharSet: crate::System::Runtime::InteropServices::CharSet,
    pub SetLastError: bool,
    pub ExactSpelling: bool,
    pub PreserveSig: bool,
    pub CallingConvention: crate::System::Runtime::InteropServices::CallingConvention,
    pub BestFitMapping: bool,
    pub ThrowOnUnmappableChar: bool,
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::DllImportAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "DllImportAttribute";
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
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::DllImportAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::DllImportAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl crate::System::Runtime::InteropServices::DllImportAttribute {
    pub fn GetCustomAttribute(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeMethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::DllImportAttribute as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::RuntimeMethodInfo,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                1usize,
            >("GetCustomAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::DllImportAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "GetCustomAttribute", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Attribute> = unsafe {
            method.invoke_unchecked((), (method))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeMethodInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::DllImportAttribute as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Reflection::RuntimeMethodInfo,
                >),
                bool,
                1usize,
            >("IsDefined")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::DllImportAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "IsDefined", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString1(
        dllName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dllName))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_CharSet__cordl_bool__cordl_bool__cordl_bool_CallingConvention__cordl_bool__cordl_bool0(
        dllName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        entryPoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charSet: crate::System::Runtime::InteropServices::CharSet,
        exactSpelling: bool,
        setLastError: bool,
        preserveSig: bool,
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
        bestFitMapping: bool,
        throwOnUnmappableChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    dllName,
                    entryPoint,
                    charSet,
                    exactSpelling,
                    setLastError,
                    preserveSig,
                    callingConvention,
                    bestFitMapping,
                    throwOnUnmappableChar,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        dllName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::DllImportAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::DllImportAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dllName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_CharSet__cordl_bool__cordl_bool__cordl_bool_CallingConvention__cordl_bool__cordl_bool0(
        &mut self,
        dllName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        entryPoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charSet: crate::System::Runtime::InteropServices::CharSet,
        exactSpelling: bool,
        setLastError: bool,
        preserveSig: bool,
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
        bestFitMapping: bool,
        throwOnUnmappableChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::DllImportAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Runtime::InteropServices::CharSet,
                    bool,
                    bool,
                    bool,
                    crate::System::Runtime::InteropServices::CallingConvention,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::DllImportAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        dllName,
                        entryPoint,
                        charSet,
                        exactSpelling,
                        setLastError,
                        preserveSig,
                        callingConvention,
                        bestFitMapping,
                        throwOnUnmappableChar,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::InteropServices::DllImportAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::InteropServices::DllImportAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "get_Value", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::DllImportAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
