#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
#[repr(C)]
#[derive(Debug)]
pub struct Function {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _functionType: crate::MS::Internal::Xml::XPath::Function_FunctionType,
    pub _argumentList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        >,
    >,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
unsafe impl quest_hook::libil2cpp::Type for crate::MS::Internal::Xml::XPath::Function {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Function";
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
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Function {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Function {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl crate::MS::Internal::Xml::XPath::Function {
    #[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
    pub type FunctionType = crate::MS::Internal::Xml::XPath::Function_FunctionType;
    pub fn New_Function_FunctionType_AstNode2(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        arg: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, arg))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Function_FunctionType_List_1_0(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, argumentList))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_List_1_1(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefix, name, argumentList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Function_FunctionType_AstNode2(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        arg: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::MS::Internal::Xml::XPath::Function_FunctionType,
                            quest_hook::libil2cpp::Gc<
                                crate::MS::Internal::Xml::XPath::AstNode,
                            >,
                        ),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ftype, arg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Function_FunctionType_List_1_0(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::MS::Internal::Xml::XPath::Function_FunctionType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::MS::Internal::Xml::XPath::AstNode,
                                    >,
                                >,
                            >,
                        ),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ftype, argumentList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_List_1_1(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::MS::Internal::Xml::XPath::AstNode,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (prefix, name, argumentList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathResultType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Xml::XPath::XPathResultType,
                        0usize,
                    >("get_ReturnType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ReturnType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::XPath::XPathResultType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::AstNode_AstType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::MS::Internal::Xml::XPath::AstNode_AstType,
                        0usize,
                    >("get_Type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::AstNode_AstType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Function {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Function_FunctionType {
    #[default]
    FuncBoolean = 8i32,
    FuncCeiling = 25i32,
    FuncConcat = 13i32,
    FuncContains = 15i32,
    FuncCount = 2i32,
    FuncFalse = 11i32,
    FuncFloor = 24i32,
    FuncID = 3i32,
    FuncLang = 22i32,
    FuncLast = 0i32,
    FuncLocalName = 4i32,
    FuncName = 6i32,
    FuncNameSpaceUri = 5i32,
    FuncNormalize = 20i32,
    FuncNot = 12i32,
    FuncNumber = 9i32,
    FuncPosition = 1i32,
    FuncRound = 26i32,
    FuncStartsWith = 14i32,
    FuncString = 7i32,
    FuncStringLength = 19i32,
    FuncSubstring = 18i32,
    FuncSubstringAfter = 17i32,
    FuncSubstringBefore = 16i32,
    FuncSum = 23i32,
    FuncTranslate = 21i32,
    FuncTrue = 10i32,
    FuncUserDefined = 27i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::XPath::Function_FunctionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Function/FunctionType";
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
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::MS::Internal::Xml::XPath::Function_FunctionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::MS::Internal::Xml::XPath::Function_FunctionType {
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
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::MS::Internal::Xml::XPath::Function_FunctionType {
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
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::MS::Internal::Xml::XPath::Function_FunctionType {
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
