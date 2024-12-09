#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _scanner: *mut crate::MS::Internal::Xml::XPath::XPathScanner,
    pub _parseDepth: i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::XPathParser =>
    "MS.Internal.Xml.XPath"."XPathParser"
);
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::XPathParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::XPathParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNodeSet", (t))?;
        Ok(__cordl_ret)
    }
    pub fn CheckToken(
        &mut self,
        t: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckToken", (t))?;
        Ok(__cordl_ret)
    }
    pub fn GetAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MS::Internal::Xml::XPath::Axis_AxisType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Axis_AxisType = __cordl_object
            .invoke("GetAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        scanner: *mut crate::MS::Internal::Xml::XPath::XPathScanner,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scanner))?;
        Ok(__cordl_object)
    }
    pub fn NextLex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextLex", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseAdditiveExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseAdditiveExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseAndExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseAndExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseEqualityExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseEqualityExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseExpression(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseExpression", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseFilterExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseFilterExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseLocationPath(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseLocationPath", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMethod(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseMethod", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMultiplicativeExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseMultiplicativeExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseNodeTest(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        nodeType: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseNodeTest", (qyInput, axisType, nodeType))?;
        Ok(__cordl_ret)
    }
    pub fn ParseOrExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseOrExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParsePathExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParsePathExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParsePredicate(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParsePredicate", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParsePrimaryExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParsePrimaryExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseRelationalExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseRelationalExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseRelativeLocationPath(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseRelativeLocationPath", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseStep(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseStep", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseUnaryExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseUnaryExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn ParseUnionExpr(
        &mut self,
        qyInput: *mut crate::MS::Internal::Xml::XPath::AstNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::MS::Internal::Xml::XPath::AstNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::MS::Internal::Xml::XPath::AstNode = __cordl_object
            .invoke("ParseUnionExpr", (qyInput))?;
        Ok(__cordl_ret)
    }
    pub fn PassToken(
        &mut self,
        t: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PassToken", (t))?;
        Ok(__cordl_ret)
    }
    pub fn TestOp(
        &mut self,
        op: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TestOp", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        scanner: *mut crate::MS::Internal::Xml::XPath::XPathScanner,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scanner))?;
        Ok(__cordl_ret)
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
    pub _argTypes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XPath::XPathResultType,
    >,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::XPathParser_ParamInfo
    => "MS.Internal.Xml.XPath"."XPathParser/ParamInfo"
);
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathParser+ParamInfo")]
impl crate::MS::Internal::Xml::XPath::XPathParser_ParamInfo {
    pub fn New(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        minargs: i32,
        maxargs: i32,
        argTypes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::XPath::XPathResultType,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, minargs, maxargs, argTypes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        minargs: i32,
        maxargs: i32,
        argTypes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::XPath::XPathResultType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ftype, minargs, maxargs, argTypes))?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::XPath::XPathResultType,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::XPath::XPathResultType,
        > = __cordl_object.invoke("get_ArgTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::Function_FunctionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Function_FunctionType = __cordl_object
            .invoke("get_FType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Maxargs(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Maxargs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Minargs(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Minargs", ())?;
        Ok(__cordl_ret)
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
