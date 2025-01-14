#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Reflection::SignatureTypeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "SignatureTypeExtensions";
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
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
impl std::ops::Deref for crate::System::Reflection::SignatureTypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
impl std::ops::DerefMut for crate::System::Reflection::SignatureTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
impl crate::System::Reflection::SignatureTypeExtensions {
    pub fn MatchesExactly(
        pattern: quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
        actual: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("MatchesExactly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatchesExactly", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (pattern, actual))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesParameterTypeExactly(
        pattern: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parameter: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
                ),
                bool,
                2usize,
            >("MatchesParameterTypeExactly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatchesParameterTypeExactly", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (pattern, parameter))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeArrayType_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("TryMakeArrayType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryMakeArrayType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeArrayType_i32_1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        rank: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>, i32),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("TryMakeArrayType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryMakeArrayType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_cordl_type, rank))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeByRefType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("TryMakeByRefType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryMakeByRefType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeGenericType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instantiation: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("TryMakeGenericType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryMakeGenericType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_cordl_type, instantiation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMakePointerType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("TryMakePointerType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryMakePointerType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryResolve(
        signatureType: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::SignatureType,
        >,
        genericMethodParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("TryResolve")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryResolve", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (signatureType, genericMethodParameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryResolveAgainstGenericMethod(
        signatureType: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::SignatureType,
        >,
        genericMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::SignatureType>,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("TryResolveAgainstGenericMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryResolveAgainstGenericMethod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (signatureType, genericMethod))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::SignatureTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
