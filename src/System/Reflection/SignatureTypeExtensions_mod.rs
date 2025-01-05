#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::SignatureTypeExtensions =>
    "System.Reflection"."SignatureTypeExtensions"
);
#[cfg(feature = "System+Reflection+SignatureTypeExtensions")]
impl std::ops::Deref for crate::System::Reflection::SignatureTypeExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesExactly", (pattern, actual))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchesParameterTypeExactly(
        pattern: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parameter: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchesParameterTypeExactly", (pattern, parameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeArrayType_Gc0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMakeArrayType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeArrayType_i32_1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        rank: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMakeArrayType", (_cordl_type, rank))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryMakeByRefType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMakeByRefType", (_cordl_type))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMakeGenericType", (_cordl_type, instantiation))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryMakePointerType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryMakePointerType", (_cordl_type))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryResolve", (signatureType, genericMethodParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryResolveAgainstGenericMethod(
        signatureType: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::SignatureType,
        >,
        genericMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryResolveAgainstGenericMethod", (signatureType, genericMethod))?;
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
