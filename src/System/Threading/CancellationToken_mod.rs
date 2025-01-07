#[cfg(feature = "System+Threading+CancellationToken")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancellationToken {
    pub _source: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
}
#[cfg(feature = "System+Threading+CancellationToken")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::CancellationToken {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationToken";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::CancellationToken {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::CancellationToken {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::CancellationToken {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::CancellationToken {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Threading+CancellationToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::CancellationToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+CancellationToken")]
impl crate::System::Threading::CancellationToken {
    pub fn Equals_CancellationToken0(
        &mut self,
        other: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterWithoutEC(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        let __cordl_ret: crate::System::Threading::CancellationTokenRegistration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InternalRegisterWithoutEC",
            (callback, state),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Register_Action0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        let __cordl_ret: crate::System::Threading::CancellationTokenRegistration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Register",
            (callback),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Register_Action_1_Il2CppObject1(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        let __cordl_ret: crate::System::Threading::CancellationTokenRegistration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Register",
            (callback, state),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Register_Action_1_Il2CppObject__cordl_bool__cordl_bool2(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        useSynchronizationContext: bool,
        useExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        let __cordl_ret: crate::System::Threading::CancellationTokenRegistration = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Register",
            (callback, state, useSynchronizationContext, useExecutionContext),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowIfCancellationRequested",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOperationCanceledException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ThrowOperationCanceledException",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CancellationTokenSource0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationTokenSource,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (source),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        canceled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (canceled),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanBeCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_CanBeCanceled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCancellationRequested",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_None() -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationToken,
    > {
        let __cordl_ret: crate::System::Threading::CancellationToken = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_None", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Threading::CancellationToken,
        right: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::System::Threading::CancellationToken,
        right: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
