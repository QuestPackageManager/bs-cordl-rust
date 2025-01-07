#[cfg(feature = "System+Data+ExprException")]
#[repr(C)]
#[derive(Debug)]
pub struct ExprException {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+ExprException")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::ExprException {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "ExprException";
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
#[cfg(feature = "System+Data+ExprException")]
impl std::ops::Deref for crate::System::Data::ExprException {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExprException")]
impl std::ops::DerefMut for crate::System::Data::ExprException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExprException")]
impl crate::System::Data::ExprException {
    pub fn AggregateArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AggregateArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AggregateUnbound(
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AggregateUnbound", (expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn AmbiguousBinop(
        op: i32,
        type1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        type2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AmbiguousBinop", (op, type1, type2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArgumentType(
        function: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArgumentType", (function, arg, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArgumentTypeInteger(
        function: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArgumentTypeInteger", (function, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindFailure(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::EvaluateException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::EvaluateException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BindFailure", (relationName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeNotAggregate(
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeNotAggregate", (expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DatatypeConvertion(
        type1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        type2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DatatypeConvertion", (type1, type2))?;
        Ok(__cordl_ret.into())
    }
    pub fn DatavalueConvertion(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DatavalueConvertion", (value, _cordl_type, innerException))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvalNoContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvalNoContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpressionTooComplex() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpressionTooComplex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpressionUnbound(
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpressionUnbound", (expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterConvertion(
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterConvertion", (expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FunctionArgumentCount(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FunctionArgumentCount", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FunctionArgumentOutOfRange(
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        func: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FunctionArgumentOutOfRange", (arg, func))?;
        Ok(__cordl_ret.into())
    }
    pub fn InWithoutList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InWithoutList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InWithoutParentheses() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InWithoutParentheses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidDate(
        date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidDate", (date))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidHoursArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidHoursArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidIsSyntax() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidIsSyntax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidMinutesArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidMinutesArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidNameBracketing(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidNameBracketing", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidPattern(
        pat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidPattern", (pat))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidString(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidString", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidTimeZoneRange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidTimeZoneRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidType(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidType", (typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookupArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MismatchKindandTimeSpan() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MismatchKindandTimeSpan", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MissingOperand(
        before: quest_hook::libil2cpp::Gc<crate::System::Data::OperatorInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MissingOperand", (before))?;
        Ok(__cordl_ret.into())
    }
    pub fn MissingOperandBefore(
        op: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MissingOperandBefore", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn MissingOperator(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MissingOperator", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn MissingRightParen() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MissingRightParen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NYI(
        moreinfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NYI", (moreinfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn NonConstantArgument() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NonConstantArgument", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Overflow(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Overflow", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyntaxError() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SyntaxError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TooManyRightParentheses() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TooManyRightParentheses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeMismatch(
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeMismatch", (expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeMismatchInBinop(
        op: i32,
        type1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        type2: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeMismatchInBinop", (op, type1, type2))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnboundName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnboundName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn UndefinedFunction(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UndefinedFunction", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnknownToken_Il2CppString_i32_0(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnknownToken", (token, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnknownToken_Tokens_Tokens_i32_1(
        tokExpected: crate::System::Data::Tokens,
        tokCurr: crate::System::Data::Tokens,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnknownToken", (tokExpected, tokCurr, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnresolvedRelation(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnresolvedRelation", (name, expr))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsupportedDataType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsupportedDataType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsupportedOperator(
        op: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsupportedOperator", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Eval_Exception1(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::EvaluateException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::EvaluateException,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_Eval", (error, innerException))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Eval_Il2CppString0(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::EvaluateException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::EvaluateException,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("_Eval", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Expr(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::InvalidExpressionException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::InvalidExpressionException,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("_Expr", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Overflow(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::OverflowException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::OverflowException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_Overflow", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Syntax(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::SyntaxErrorException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::SyntaxErrorException,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("_Syntax", (error))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+ExprException")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ExprException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
