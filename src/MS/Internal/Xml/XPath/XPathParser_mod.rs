#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _scanner: quest_hook::libil2cpp::Gc<
        crate::MS::Internal::Xml::XPath::XPathScanner,
    >,
    pub _parseDepth: i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::XPath::XPathParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "XPathParser";
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
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::XPathParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::XPathParser {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl crate::MS::Internal::Xml::XPath::XPathParser {
    #[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
    pub type ParamInfo = crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo;
    pub fn CheckNodeSet(
        &mut self,
        t: crate::System::Xml::XPath::XPathResultType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Xml::XPath::XPathResultType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckNodeSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckNodeSet", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckToken(
        &mut self,
        t: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::MS::Internal::Xml::XPath::XPathScanner_LexKind),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckToken", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAxesTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::MS::Internal::Xml::XPath::Axis_AxisType,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                                crate::MS::Internal::Xml::XPath::Axis_AxisType,
                            >,
                        >,
                        0usize,
                    >("CreateAxesTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateAxesTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::MS::Internal::Xml::XPath::Axis_AxisType,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFunctionTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo,
                                >,
                            >,
                        >,
                        0usize,
                    >("CreateFunctionTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateFunctionTable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MS::Internal::Xml::XPath::Axis_AxisType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::MS::Internal::Xml::XPath::Axis_AxisType,
                        0usize,
                    >("GetAxis")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAxis", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Axis_AxisType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNodeType(
        scaner: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::XPathScanner>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::XPathScanner,
                        >),
                        bool,
                        1usize,
                    >("IsNodeType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsNodeType", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (scaner))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimaryExpr(
        scanner: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::XPathScanner>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::XPathScanner,
                        >),
                        bool,
                        1usize,
                    >("IsPrimaryExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsPrimaryExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (scanner))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsStep(
        lexKind: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::MS::Internal::Xml::XPath::XPathScanner_LexKind),
                        bool,
                        1usize,
                    >("IsStep")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsStep", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (lexKind))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scanner: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::XPathScanner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scanner))?;
        Ok(__cordl_object.into())
    }
    pub fn NextLex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("NextLex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NextLex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseAdditiveExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseAdditiveExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseAdditiveExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseAndExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseAndExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseAndExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseEqualityExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseEqualityExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseEqualityExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExpression(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseExpression")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseExpression", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseFilterExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseFilterExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseFilterExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseLocationPath(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseLocationPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseLocationPath", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseMethod(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseMethod", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseMultiplicativeExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseMultiplicativeExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseMultiplicativeExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseNodeTest(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        nodeType: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::MS::Internal::Xml::XPath::AstNode,
                            >,
                            crate::MS::Internal::Xml::XPath::Axis_AxisType,
                            crate::System::Xml::XPath::XPathNodeType,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        3usize,
                    >("ParseNodeTest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseNodeTest", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput, axisType, nodeType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseOrExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseOrExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseOrExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParsePathExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParsePathExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParsePathExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParsePredicate(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParsePredicate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParsePredicate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParsePrimaryExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParsePrimaryExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParsePrimaryExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseRelationalExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseRelationalExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseRelationalExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseRelativeLocationPath(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseRelativeLocationPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseRelativeLocationPath", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseStep(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseStep")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseStep", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnaryExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseUnaryExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseUnaryExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnionExpr(
        &mut self,
        qyInput: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseUnionExpr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseUnionExpr", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked(self, (qyInput))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseXPathExpression(
        xpathExpression: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::AstNode,
                        >,
                        1usize,
                    >("ParseXPathExpression")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ParseXPathExpression", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = unsafe { method.invoke_unchecked((), (xpathExpression))? };
        Ok(__cordl_ret.into())
    }
    pub fn PassToken(
        &mut self,
        t: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::MS::Internal::Xml::XPath::XPathScanner_LexKind),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PassToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PassToken", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TestOp(
        &mut self,
        op: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("TestOp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TestOp", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (op))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        scanner: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::XPathScanner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::MS::Internal::Xml::XPath::XPathScanner,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (scanner))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::XPathParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathParser_ParamInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
    pub _minargs: i32,
    pub _maxargs: i32,
    pub _argTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Xml::XPath::XPathResultType>,
    >,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "MS.Internal.Xml.XPath";
    const CLASS_NAME: &'static str = "XPathParser/ParamInfo";
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
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    pub fn New(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        minargs: i32,
        maxargs: i32,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::XPath::XPathResultType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, minargs, maxargs, argTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        minargs: i32,
        maxargs: i32,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::XPath::XPathResultType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::MS::Internal::Xml::XPath::Function_FunctionType,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::System::Xml::XPath::XPathResultType,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ftype, minargs, maxargs, argTypes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::XPath::XPathResultType,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::System::Xml::XPath::XPathResultType,
                            >,
                        >,
                        0usize,
                    >("get_ArgTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_ArgTypes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Xml::XPath::XPathResultType,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_FType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::Function_FunctionType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::MS::Internal::Xml::XPath::Function_FunctionType,
                        0usize,
                    >("get_FType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_FType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Function_FunctionType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Maxargs(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_Maxargs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Maxargs", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Minargs(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_Minargs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Minargs", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
