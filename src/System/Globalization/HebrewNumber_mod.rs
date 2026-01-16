#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber")]
#[repr(C)]
#[derive(Debug)]
pub struct HebrewNumber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::HebrewNumber {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "HebrewNumber";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl std::ops::Deref for crate::System::Globalization::HebrewNumber {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl std::ops::DerefMut for crate::System::Globalization::HebrewNumber {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber")]
impl crate::System::Globalization::HebrewNumber {
    #[cfg(feature = "System+Globalization+HebrewNumber+HS")]
    pub type HS = crate::System::Globalization::HebrewNumber_HS;
    #[cfg(feature = "System+Globalization+HebrewNumber+HebrewToken")]
    pub type HebrewToken = crate::System::Globalization::HebrewNumber_HebrewToken;
    #[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
    pub type HebrewValue = crate::System::Globalization::HebrewNumber_HebrewValue;
    pub fn IsDigit(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), bool, 1usize>("IsDigit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsDigit",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseByChar(
        ch: char,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::HebrewNumberParsingContext,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::HebrewNumberParsingState> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        char,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Globalization::HebrewNumberParsingContext,
                        >,
                    ), crate::System::Globalization::HebrewNumberParsingState, 2usize>(
                        "ParseByChar",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseByChar",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::HebrewNumberParsingState =
            unsafe { cordl_method_info.invoke_unchecked((), (ch, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        Number: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (Number))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::HebrewNumber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i8)]
pub enum HebrewNumber_HS {
    #[default]
    END = 100i8,
    S400 = 1i8,
    S400_400 = 2i8,
    S400_400_100 = 13i8,
    S400_400_DQ = 12i8,
    S400_DQ = 11i8,
    S400_X0 = 4i8,
    S400_X00 = 3i8,
    S400_X00_X0 = 6i8,
    S9 = 14i8,
    S9_DQ = 16i8,
    Start = 0i8,
    X = 8i8,
    X0 = 9i8,
    X00 = 10i8,
    X00_DQ = 5i8,
    X00_S9 = 15i8,
    X0_DQ = 7i8,
    _err = -1i8,
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::HebrewNumber_HS {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "HebrewNumber/HS";
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Globalization::HebrewNumber_HS {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Globalization::HebrewNumber_HS {
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Globalization::HebrewNumber_HS {
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HS")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Globalization::HebrewNumber_HS {
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i16)]
pub enum HebrewNumber_HebrewToken {
    #[default]
    Digit1 = 4i16,
    Digit10 = 3i16,
    Digit100 = 2i16,
    Digit200_300 = 1i16,
    Digit400 = 0i16,
    Digit6_7 = 5i16,
    Digit7 = 6i16,
    Digit9 = 7i16,
    DoubleQuote = 9i16,
    Invalid = -1i16,
    SingleQuote = 8i16,
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::HebrewNumber_HebrewToken {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "HebrewNumber/HebrewToken";
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Globalization::HebrewNumber_HebrewToken
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Globalization::HebrewNumber_HebrewToken
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Globalization::HebrewNumber_HebrewToken
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewToken")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Globalization::HebrewNumber_HebrewToken
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct HebrewNumber_HebrewValue {
    pub token: crate::System::Globalization::HebrewNumber_HebrewToken,
    pub value: i16,
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::HebrewNumber_HebrewValue {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "HebrewNumber/HebrewValue";
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Globalization::HebrewNumber_HebrewValue
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Globalization::HebrewNumber_HebrewValue
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Globalization::HebrewNumber_HebrewValue
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Globalization::HebrewNumber_HebrewValue
{
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
#[cfg(feature = "cordl_class_System+Globalization+HebrewNumber+HebrewValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::System::Globalization::HebrewNumber_HebrewValue
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumber+HebrewValue")]
impl crate::System::Globalization::HebrewNumber_HebrewValue {
    pub fn _ctor(
        &mut self,
        token: crate::System::Globalization::HebrewNumber_HebrewToken,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::HebrewNumber_HebrewToken, i16),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (token, value))? };
        Ok(__cordl_ret.into())
    }
}
