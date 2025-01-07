#[cfg(feature = "System+Data+Common+ADP")]
#[repr(C)]
#[derive(Debug)]
pub struct ADP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+Common+ADP")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Common::ADP {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "ADP";
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
#[cfg(feature = "System+Data+Common+ADP")]
impl std::ops::Deref for crate::System::Data::Common::ADP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ADP")]
impl std::ops::DerefMut for crate::System::Data::Common::ADP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ADP")]
impl crate::System::Data::Common::ADP {
    pub fn Argument(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::ArgumentException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Argument", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArgumentOutOfRange_Il2CppString0(
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArgumentOutOfRange", (parameterName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArgumentOutOfRange_Il2CppString1(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArgumentOutOfRange", (message, parameterName))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidAcceptRejectRule(
        value: crate::System::Data::AcceptRejectRule,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidAcceptRejectRule", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidEnumerationValue(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidEnumerationValue", (_cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidMissingSchemaAction(
        value: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidMissingSchemaAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidOperation(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::InvalidOperationException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::InvalidOperationException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidOperation", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidRule(
        value: crate::System::Data::Rule,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ArgumentOutOfRangeException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ArgumentOutOfRangeException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidRule", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidSeekOrigin(
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidSeekOrigin", (parameterName))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCatchableExceptionType(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCatchableExceptionType", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCatchableOrSecurityExceptionType(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCatchableOrSecurityExceptionType", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotSupported(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::NotSupportedException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::NotSupportedException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotSupported", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceException(
        trace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceException", (trace, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceExceptionAsReturnValue(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceExceptionAsReturnValue", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraceExceptionWithoutRethrow(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TraceExceptionWithoutRethrow", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrongType(
        got: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expected: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrongType", (got, expected))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Common+ADP")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::ADP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
