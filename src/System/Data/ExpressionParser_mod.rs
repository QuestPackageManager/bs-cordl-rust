#[cfg(feature = "System+Data+ExpressionParser")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _escape: char,
    pub _decimalSeparator: char,
    pub _listSeparator: char,
    pub _exponentL: char,
    pub _exponentU: char,
    pub _text: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _pos: i32,
    pub _start: i32,
    pub _token: crate::System::Data::Tokens,
    pub _op: i32,
    pub _ops: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::OperatorInfo,
    >,
    pub _topOperator: i32,
    pub _topNode: i32,
    pub _table: *mut crate::System::Data::DataTable,
    pub _nodeStack: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::ExpressionNode,
    >,
    pub _prevOperand: i32,
    pub _expression: *mut crate::System::Data::ExpressionNode,
}
#[cfg(feature = "System+Data+ExpressionParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ExpressionParser => "System.Data"
    ."ExpressionParser"
);
#[cfg(feature = "System+Data+ExpressionParser")]
impl std::ops::Deref for crate::System::Data::ExpressionParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExpressionParser")]
impl std::ops::DerefMut for crate::System::Data::ExpressionParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExpressionParser")]
impl crate::System::Data::ExpressionParser {
    #[cfg(feature = "System+Data+ExpressionParser+ReservedWords")]
    pub type ReservedWords = crate::System::Data::ExpressionParser_ReservedWords;
    pub fn BuildExpression(
        &mut self,
        pri: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildExpression", (pri))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckToken(
        &mut self,
        token: crate::System::Data::Tokens,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAlpha(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAlpha", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAlphaNumeric(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAlphaNumeric", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDigit", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWhiteSpace", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadExpression(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadExpression", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object.into())
    }
    pub fn NodePeek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = __cordl_object.invoke("NodePeek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NodePop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = __cordl_object.invoke("NodePop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NodePush(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NodePush", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = __cordl_object.invoke("Parse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAggregateArgument(
        &mut self,
        aggregate: crate::System::Data::FunctionId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ExpressionNode,
        > = __cordl_object.invoke("ParseAggregateArgument", (aggregate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scan(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Tokens> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Tokens = __cordl_object
            .invoke("Scan", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanBinaryConstant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanBinaryConstant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanDate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanName_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanName__cordl_char__cordl_char_Il2CppString1(
        &mut self,
        chEnd: char,
        esc: char,
        charsToEscape: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanName", (chEnd, esc, charsToEscape))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNumeric(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanNumeric", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanReserved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanReserved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanString(
        &mut self,
        escape: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanString", (escape))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanToken(
        &mut self,
        token: crate::System::Data::Tokens,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanWhite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanWhite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartScan(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartScan", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+ExpressionParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ExpressionParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+ExpressionParser+ReservedWords")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExpressionParser_ReservedWords {
    pub _word: *mut quest_hook::libil2cpp::Il2CppString,
    pub _token: crate::System::Data::Tokens,
    pub _op: i32,
}
#[cfg(feature = "System+Data+ExpressionParser+ReservedWords")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ExpressionParser_ReservedWords =>
    "System.Data"."ExpressionParser/ReservedWords"
);
#[cfg(feature = "System+Data+ExpressionParser+ReservedWords")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::ExpressionParser_ReservedWords {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+ExpressionParser+ReservedWords")]
impl crate::System::Data::ExpressionParser_ReservedWords {
    pub fn _ctor(
        &mut self,
        word: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Data::Tokens,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (word, token, op),
        )?;
        Ok(__cordl_ret.into())
    }
}
