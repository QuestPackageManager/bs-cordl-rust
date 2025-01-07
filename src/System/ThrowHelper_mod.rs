#[cfg(feature = "System+ThrowHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ThrowHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ThrowHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ThrowHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ThrowHelper";
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
#[cfg(feature = "System+ThrowHelper")]
impl std::ops::Deref for crate::System::ThrowHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ThrowHelper")]
impl std::ops::DerefMut for crate::System::ThrowHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ThrowHelper")]
impl crate::System::ThrowHelper {
    pub fn CreateArgumentException_DestinationTooShort() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgumentException_DestinationTooShort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgumentNullException(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgumentNullException", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgumentOutOfRangeException_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgumentOutOfRangeException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgumentOutOfRangeException_ExceptionArgument1(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgumentOutOfRangeException", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArrayTypeMismatchException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArrayTypeMismatchException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateIndexOutOfRangeException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateIndexOutOfRangeException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThrowNotSupportedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateThrowNotSupportedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAddingDuplicateWithKeyArgumentException(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::ArgumentException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAddingDuplicateWithKeyArgumentException", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgumentException(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::ArgumentException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArgumentException", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgumentName(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArgumentName", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgumentNullException(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentNullException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentNullException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArgumentNullException", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgumentOutOfRangeException_ExceptionResource1(
        argument: crate::System::ExceptionArgument,
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArgumentOutOfRangeException", (argument, resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArgumentOutOfRangeException_Il2CppString0(
        argument: crate::System::ExceptionArgument,
        resource: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArgumentOutOfRangeException", (argument, resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArraySegmentCtorValidationFailedException(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetArraySegmentCtorValidationFailedException",
                (array, offset, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvalidOperationException(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::InvalidOperationException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::InvalidOperationException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvalidOperationException", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyNotFoundException(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyNotFoundException,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyNotFoundException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyNotFoundException", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceName(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResourceName", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn IfNullAndNullsAreIllegalThenThrow<T>(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        argName: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IfNullAndNullsAreIllegalThenThrow", (value, argName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAddingDuplicateWithKeyArgumentException(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAddingDuplicateWithKeyArgumentException", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentException_Argument_InvalidArrayType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentException_Argument_InvalidArrayType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentException_DestinationTooShort() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentException_DestinationTooShort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentException_ExceptionArgument1(
        resource: crate::System::ExceptionResource,
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentException", (resource, argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentException_ExceptionResource0(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentException", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentNullException(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentNullException", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentOutOfRangeException_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentOutOfRangeException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentOutOfRangeException_ExceptionArgument1(
        argument: crate::System::ExceptionArgument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentOutOfRangeException", (argument))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentOutOfRangeException_ExceptionArgument_ExceptionResource2(
        argument: crate::System::ExceptionArgument,
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentOutOfRangeException", (argument, resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArgumentOutOfRange_IndexException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArgumentOutOfRange_IndexException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArraySegmentCtorValidationFailedExceptions(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowArraySegmentCtorValidationFailedExceptions",
                (array, offset, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowArrayTypeMismatchException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowArrayTypeMismatchException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowCountArgumentOutOfRange_ArgumentOutOfRange_Count() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowCountArgumentOutOfRange_ArgumentOutOfRange_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIndexArgumentOutOfRange_NeedNonNegNumException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowIndexArgumentOutOfRange_NeedNonNegNumException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIndexOutOfRangeException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowIndexOutOfRangeException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInvalidOperationException", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_ConcurrentOperationsNotSupported() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowInvalidOperationException_ConcurrentOperationsNotSupported",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_InvalidOperation_EnumEnded() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInvalidOperationException_InvalidOperation_EnumEnded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_InvalidOperation_EnumFailedVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowInvalidOperationException_InvalidOperation_EnumFailedVersion",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_InvalidOperation_EnumNotStarted() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowInvalidOperationException_InvalidOperation_EnumNotStarted",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_InvalidOperation_EnumOpCantHappen() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowInvalidOperationException_InvalidOperation_EnumOpCantHappen",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidOperationException_InvalidOperation_NoValue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInvalidOperationException_InvalidOperation_NoValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidTypeWithPointersNotSupported(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowInvalidTypeWithPointersNotSupported", (targetType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowKeyNotFoundException(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowKeyNotFoundException", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowNotSupportedException_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowNotSupportedException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowNotSupportedException_ExceptionResource1(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowNotSupportedException", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowSerializationException(
        resource: crate::System::ExceptionResource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowSerializationException", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowStartIndexArgumentOutOfRange_ArgumentOutOfRange_Index() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowStartIndexArgumentOutOfRange_ArgumentOutOfRange_Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowWrongKeyTypeArgumentException(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowWrongKeyTypeArgumentException", (key, targetType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowWrongValueTypeArgumentException(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowWrongValueTypeArgumentException", (value, targetType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ThrowHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ThrowHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
