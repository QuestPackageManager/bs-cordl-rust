#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegexWriter {
    pub _emitted: crate::System::Collections::Generic::ValueListBuilder_1<i32>,
    pub _intStack: crate::System::Collections::Generic::ValueListBuilder_1<i32>,
    pub _stringHash: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            i32,
        >,
    >,
    pub _stringTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub _trackCount: i32,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::RegexWriter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "RegexWriter";
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
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Text::RegularExpressions::RegexWriter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Text::RegularExpressions::RegexWriter {
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
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Text::RegularExpressions::RegexWriter {
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
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Text::RegularExpressions::RegexWriter {
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
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::RegexWriter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
impl crate::System::Text::RegularExpressions::RegexWriter {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EmitFragment(
        &mut self,
        nodetype: i32,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        curIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EmitFragment",
            (nodetype, node, curIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_i32_0(
        &mut self,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Emit",
            (op),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_i32_1(
        &mut self,
        op: i32,
        opd1: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Emit",
            (op, opd1),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Emit_i32_i32_2(
        &mut self,
        op: i32,
        opd1: i32,
        opd2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Emit",
            (op, opd1, opd2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MapCapnum(&mut self, capnum: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MapCapnum",
            (capnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PatchJump(
        &mut self,
        offset: i32,
        jumpDest: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PatchJump",
            (offset, jumpDest),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegexCodeFromRegexTree(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexTree,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexCode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RegexCodeFromRegexTree",
            (tree),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StringCode(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StringCode",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        tree: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexTree,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexCode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Write", (tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        emittedSpan: crate::System::Span_1<i32>,
        intStackSpan: crate::System::Span_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (emittedSpan, intStackSpan),
        )?;
        Ok(__cordl_ret.into())
    }
}
