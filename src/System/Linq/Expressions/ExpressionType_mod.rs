#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpressionType {
    #[default]
    Add = 0i32,
    AddAssign = 63i32,
    AddAssignChecked = 74i32,
    AddChecked = 1i32,
    And = 2i32,
    AndAlso = 3i32,
    AndAssign = 64i32,
    ArrayIndex = 5i32,
    ArrayLength = 4i32,
    Assign = 46i32,
    Block = 47i32,
    Call = 6i32,
    Coalesce = 7i32,
    Conditional = 8i32,
    Constant = 9i32,
    Convert = 10i32,
    ConvertChecked = 11i32,
    DebugInfo = 48i32,
    Decrement = 49i32,
    Default = 51i32,
    Divide = 12i32,
    DivideAssign = 65i32,
    Dynamic = 50i32,
    Equal = 13i32,
    ExclusiveOr = 14i32,
    ExclusiveOrAssign = 66i32,
    Extension = 52i32,
    Goto = 53i32,
    GreaterThan = 15i32,
    GreaterThanOrEqual = 16i32,
    Increment = 54i32,
    Index = 55i32,
    Invoke = 17i32,
    IsFalse = 84i32,
    IsTrue = 83i32,
    Label = 56i32,
    Lambda = 18i32,
    LeftShift = 19i32,
    LeftShiftAssign = 67i32,
    LessThan = 20i32,
    LessThanOrEqual = 21i32,
    ListInit = 22i32,
    Loop = 58i32,
    MemberAccess = 23i32,
    MemberInit = 24i32,
    Modulo = 25i32,
    ModuloAssign = 68i32,
    Multiply = 26i32,
    MultiplyAssign = 69i32,
    MultiplyAssignChecked = 75i32,
    MultiplyChecked = 27i32,
    Negate = 28i32,
    NegateChecked = 30i32,
    New = 31i32,
    NewArrayBounds = 33i32,
    NewArrayInit = 32i32,
    Not = 34i32,
    NotEqual = 35i32,
    OnesComplement = 82i32,
    Or = 36i32,
    OrAssign = 70i32,
    OrElse = 37i32,
    Parameter = 38i32,
    PostDecrementAssign = 80i32,
    PostIncrementAssign = 79i32,
    Power = 39i32,
    PowerAssign = 71i32,
    PreDecrementAssign = 78i32,
    PreIncrementAssign = 77i32,
    Quote = 40i32,
    RightShift = 41i32,
    RightShiftAssign = 72i32,
    RuntimeVariables = 57i32,
    Subtract = 42i32,
    SubtractAssign = 73i32,
    SubtractAssignChecked = 76i32,
    SubtractChecked = 43i32,
    Switch = 59i32,
    Throw = 60i32,
    Try = 61i32,
    TypeAs = 44i32,
    TypeEqual = 81i32,
    TypeIs = 45i32,
    UnaryPlus = 29i32,
    Unbox = 62i32,
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::ExpressionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ExpressionType";
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
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Linq::Expressions::ExpressionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Linq::Expressions::ExpressionType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Linq::Expressions::ExpressionType {
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
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Linq::Expressions::ExpressionType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
