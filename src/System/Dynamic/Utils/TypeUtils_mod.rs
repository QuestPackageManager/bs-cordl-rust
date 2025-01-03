#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::TypeUtils =>
    "System.Dynamic.Utils"."TypeUtils"
);
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::TypeUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::TypeUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl crate::System::Dynamic::Utils::TypeUtils {
    #[cfg(feature = "System+Dynamic+Utils+TypeUtils+__c")]
    pub type __c = crate::System::Dynamic::Utils::TypeUtils___c;
    pub fn AreEquivalent(
        t1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        t2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEquivalent", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreReferenceAssignable(
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
        src: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreReferenceAssignable", (dest, src))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindConversionOperator(
        methods: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodInfo,
            >,
        >,
        typeFrom: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeTo: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindConversionOperator", (methods, typeFrom, typeTo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindGenericType(
        definition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindGenericType", (definition, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBooleanOperator(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBooleanOperator", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvokeMethod(
        delegateType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvokeMethod", (delegateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonNullableType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonNullableType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonRefType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonRefType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNullableType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNullableType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDefinedCoercionMethod(
        convertFrom: quest_hook::libil2cpp::Gc<crate::System::Type>,
        convertToType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUserDefinedCoercionMethod", (convertFrom, convertToType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasArrayToInterfaceConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasArrayToInterfaceConversion", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasBuiltInEqualityOperator(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasBuiltInEqualityOperator", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasIdentityPrimitiveOrNullableConversionTo(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasIdentityPrimitiveOrNullableConversionTo", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasInterfaceToArrayConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInterfaceToArrayConversion", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasReferenceConversionTo(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasReferenceConversionTo", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasReferenceEquality(
        left: quest_hook::libil2cpp::Gc<crate::System::Type>,
        right: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasReferenceEquality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsArithmetic(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsArithmetic", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBool(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBool", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsContravariant(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsContravariant", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsConvertible(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConvertible", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCovariant(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCovariant", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDelegate(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDelegate", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsImplicitBoxingConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsImplicitBoxingConversion", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsImplicitNullableConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsImplicitNullableConversion", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsImplicitNumericConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsImplicitNumericConversion", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsImplicitReferenceConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsImplicitReferenceConversion", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsImplicitlyConvertibleTo(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destination: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsImplicitlyConvertibleTo", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInteger(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInteger", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIntegerOrBool(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIntegerOrBool", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInvariant(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInvariant", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLegalExplicitVariantDelegateConversion(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLegalExplicitVariantDelegateConversion", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullableOrReferenceType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullableOrReferenceType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullableType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullableType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumeric(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumeric", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumericOrBool(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumericOrBool", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSameOrSubclass(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSameOrSubclass", (_cordl_type, subType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUnsignedInt(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUnsignedInt", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidInstanceType(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        instanceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidInstanceType", (member, instanceType))?;
        Ok(__cordl_ret.into())
    }
    pub fn StrictHasReferenceConversionTo(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        dest: quest_hook::libil2cpp::Gc<crate::System::Type>,
        skipNonArray: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StrictHasReferenceConversionTo", (source, dest, skipNonArray))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateType_Type_Il2CppString0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateType", (_cordl_type, paramName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateType__cordl_bool__cordl_bool1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allowByRef: bool,
        allowPointer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateType", (_cordl_type, paramName, allowByRef, allowPointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateType_i32_2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateType", (_cordl_type, paramName, index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::Utils::TypeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
