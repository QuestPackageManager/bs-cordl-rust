#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator")]
#[repr(C)]
#[derive(Debug)]
pub struct Operator {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _opType: crate::MS::Internal::Xml::XPath::Operator_Op,
    pub _opnd1: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    pub _opnd2: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
}
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator")]
unsafe impl quest_hook::libil2cpp::Type for crate::MS::Internal::Xml::XPath::Operator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Operator";
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
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Operator {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Operator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl crate::MS::Internal::Xml::XPath::Operator {
    #[cfg(feature = "MS+Internal+Xml+XPath+Operator+Op")]
    pub type Op = crate::MS::Internal::Xml::XPath::Operator_Op;
    pub fn New(
        op: crate::MS::Internal::Xml::XPath::Operator_Op,
        opnd1: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        opnd2: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (op, opnd1, opnd2))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        op: crate::MS::Internal::Xml::XPath::Operator_Op,
        opnd1: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        opnd2: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::MS::Internal::Xml::XPath::Operator_Op,
                            quest_hook::libil2cpp::Gc<
                                crate::MS::Internal::Xml::XPath::AstNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::MS::Internal::Xml::XPath::AstNode,
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
            cordl_method_info.invoke_unchecked(self, (op, opnd1, opnd2))?
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
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Operator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Operator_Op {
    #[default]
    AND = 2i32,
    DIV = 12i32,
    EQ = 3i32,
    GE = 8i32,
    GT = 7i32,
    INVALID = 0i32,
    LE = 6i32,
    LT = 5i32,
    MINUS = 10i32,
    _cordl_MOD = 13i32,
    MUL = 11i32,
    NE = 4i32,
    OR = 1i32,
    PLUS = 9i32,
    UNION = 14i32,
}
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::XPath::Operator_Op {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "Operator/Op";
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
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::MS::Internal::Xml::XPath::Operator_Op {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::MS::Internal::Xml::XPath::Operator_Op {
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
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::MS::Internal::Xml::XPath::Operator_Op {
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
#[cfg(feature = "cordl_class_MS+Internal+Xml+XPath+Operator+Op")]
unsafe impl quest_hook::libil2cpp::Return
for crate::MS::Internal::Xml::XPath::Operator_Op {
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
