#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum PrefixHandleType {
    #[default]
    A = 1i32,
    B = 2i32,
    Buffer = 27i32,
    C = 3i32,
    D = 4i32,
    E = 5i32,
    Empty = 0i32,
    F = 6i32,
    G = 7i32,
    H = 8i32,
    I = 9i32,
    J = 10i32,
    K = 11i32,
    L = 12i32,
    M = 13i32,
    Max = 28i32,
    N = 14i32,
    O = 15i32,
    P = 16i32,
    Q = 17i32,
    R = 18i32,
    S = 19i32,
    T = 20i32,
    U = 21i32,
    V = 22i32,
    W = 23i32,
    X = 24i32,
    Y = 25i32,
    Z = 26i32,
}
#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::PrefixHandleType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "PrefixHandleType";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::PrefixHandleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::PrefixHandleType {
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
#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::PrefixHandleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_System+Xml+PrefixHandleType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::PrefixHandleType {
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
