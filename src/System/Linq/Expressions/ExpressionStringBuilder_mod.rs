#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionStringBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionStringBuilder {
    __cordl_parent: crate::System::Linq::Expressions::ExpressionVisitor,
    pub _out: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _ids: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            i32,
        >,
    >,
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionStringBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Linq::Expressions::ExpressionStringBuilder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ExpressionStringBuilder";
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
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl std::ops::Deref for crate::System::Linq::Expressions::ExpressionStringBuilder {
    type Target = crate::System::Linq::Expressions::ExpressionVisitor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ExpressionStringBuilder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ExpressionStringBuilder")]
impl crate::System::Linq::Expressions::ExpressionStringBuilder {
    pub fn CatchBlockToString(
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::CatchBlock,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("CatchBlockToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CatchBlockToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn DumpLabel(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::LabelTarget,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DumpLabel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DumpLabel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (target))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExpressionToString(
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ExpressionToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpressionToString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetId(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        i32,
                        1usize,
                    >("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GetId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLabelId(
        &mut self,
        label: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::LabelTarget,
                        >),
                        i32,
                        1usize,
                    >("GetLabelId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLabelId", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (label))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetParamId(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::ParameterExpression,
                    >), i32, 1usize>("GetParamId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetParamId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (p))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsBool(
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >),
                        bool,
                        1usize,
                    >("IsBool")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsBool",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OutMember(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
                    ), quest_hook::libil2cpp::Void, 2usize>("OutMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OutMember",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instance, member))? };
        Ok(__cordl_ret.into())
    }
    pub fn Out_Il2CppString0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Out")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Out",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn Out__cordl_char1(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(char), quest_hook::libil2cpp::Void, 1usize>("Out")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Out",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitBinary(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BinaryExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::BinaryExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitBinary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitBinary", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::BlockExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitBlock", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitCatchBlock(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::CatchBlock,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::CatchBlock,
                        >,
                        1usize,
                    >("VisitCatchBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitCatchBlock", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::CatchBlock> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitConditional(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConditionalExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ConditionalExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitConditional")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitConditional", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitConstant(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ConstantExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitConstant")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitConstant", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitDefault(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::DefaultExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::DefaultExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitExpressions_Il2CppString1<T>(
        &mut self,
        open: char,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        >,
        close: char,
        seperator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        char,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
                        >,
                        char,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 4usize>("VisitExpressions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitExpressions",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (open, expressions, close, seperator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VisitExpressions__cordl_char_ReadOnlyCollection_1__cordl_char0<T>(
        &mut self,
        open: char,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
        >,
        close: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        char,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
                        >,
                        char,
                    ), quest_hook::libil2cpp::Void, 3usize>("VisitExpressions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "VisitExpressions",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (open, expressions, close))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitExtension(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitExtension")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitExtension", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitGoto(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::GotoExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::GotoExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitGoto")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitGoto", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitIndex(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IndexExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::IndexExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitInvocation(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::InvocationExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::InvocationExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitInvocation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitInvocation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitLabel(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::LabelExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitLabel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitLabel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitLambda<T>(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression_1<T>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    >
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression_1<T>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitLambda")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitLambda", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitLoop(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LoopExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::LoopExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitLoop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitLoop", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitMember(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MemberExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::MemberExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitMember")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitMember", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitMethodCall(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::MethodCallExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitMethodCall")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitMethodCall", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitNewArray(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::NewArrayExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitNewArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitNewArray", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitParameter(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ParameterExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::ParameterExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitParameter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitParameter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitTry(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TryExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::TryExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitTry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitTry", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitTypeBinary(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::TypeBinaryExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::TypeBinaryExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitTypeBinary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitTypeBinary", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn VisitUnary(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::UnaryExpression>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::UnaryExpression,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::Expressions::Expression,
                        >,
                        1usize,
                    >("VisitUnary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VisitUnary", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression> =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+ExpressionStringBuilder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Linq::Expressions::ExpressionStringBuilder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
