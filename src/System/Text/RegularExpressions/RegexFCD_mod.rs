#[cfg(feature = "System+Text+RegularExpressions+RegexFCD")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RegexFCD {
    pub _fcStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexFC>,
        >,
    >,
    pub _intStack: crate::System::Collections::Generic::ValueListBuilder_1<i32>,
    pub _skipAllChildren: bool,
    pub _skipchild: bool,
    pub _failed: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFCD")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexFCD =>
    "System.Text.RegularExpressions"."RegexFCD"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexFCD")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::RegexFCD {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexFCD")]
impl crate::System::Text::RegularExpressions::RegexFCD {
    pub fn AnchorFromType(_cordl_type: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnchorFromType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Anchors(
        tree: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexTree,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Anchors", (tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFC(
        &mut self,
        NodeType: i32,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        CurIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CalculateFC",
            (NodeType, node, CurIndex),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn FCIsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FCIsEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstChars(
        t: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexTree>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::Text::RegularExpressions::RegexPrefix>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::System::Text::RegularExpressions::RegexPrefix,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FirstChars", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntIsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntIsEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopFC(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexFC>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexFC,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "PopFC", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PopInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PopInt",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Prefix(
        tree: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexTree,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Text::RegularExpressions::RegexPrefix,
    > {
        let __cordl_ret: crate::System::Text::RegularExpressions::RegexPrefix = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Prefix", (tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn PushFC(
        &mut self,
        fc: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexFC>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushFC",
            (fc),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PushInt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushInt",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegexFCFromRegexTree(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexTree,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexFC>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexFC,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RegexFCFromRegexTree",
            (tree),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipChild",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TopFC(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::RegexFC>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexFC,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "TopFC", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        intStack: crate::System::Span_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (intStack),
        )?;
        Ok(__cordl_ret.into())
    }
}
