#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RegexWriter {
    pub _emitted: crate::System::Collections::Generic::ValueListBuilder_1<i32>,
    pub _intStack: crate::System::Collections::Generic::ValueListBuilder_1<i32>,
    pub _stringHash: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub _stringTable: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _caps: *mut crate::System::Collections::Hashtable,
    pub _trackCount: i32,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexWriter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexWriter =>
    "System.Text.RegularExpressions"."RegexWriter"
);
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
        Ok(__cordl_ret)
    }
    pub fn EmitFragment(
        &mut self,
        nodetype: i32,
        node: *mut crate::System::Text::RegularExpressions::RegexNode,
        curIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EmitFragment",
            (nodetype, node, curIndex),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn MapCapnum(&mut self, capnum: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MapCapnum",
            (capnum),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn RegexCodeFromRegexTree(
        &mut self,
        tree: *mut crate::System::Text::RegularExpressions::RegexTree,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexCode,
    > {
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RegexCodeFromRegexTree",
            (tree),
        )?;
        Ok(__cordl_ret)
    }
    pub fn StringCode(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StringCode",
            (str),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}