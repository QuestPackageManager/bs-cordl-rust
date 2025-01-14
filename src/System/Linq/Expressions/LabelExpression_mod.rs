#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Target_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::LabelTarget,
    >,
    pub _DefaultValue_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::LabelExpression {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "LabelExpression";
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
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::LabelExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::LabelExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl crate::System::Linq::Expressions::LabelExpression {
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ExpressionVisitor,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                1usize,
            >("Accept")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Accept", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, (visitor)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        label: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        defaultValue: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, defaultValue))?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        defaultValue: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelExpression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::LabelTarget,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::LabelExpression,
                >,
                2usize,
            >("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelExpression,
        > = unsafe { method.invoke_unchecked(self, (target, defaultValue)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        label: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        defaultValue: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::LabelTarget,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Linq::Expressions::Expression,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (label, defaultValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
                0usize,
            >("get_DefaultValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_DefaultValue", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Linq::Expressions::ExpressionType,
                0usize,
            >("get_NodeType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_NodeType", 0usize
                )
            });
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
                0usize,
            >("get_Target")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Target", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Type", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+LabelExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::LabelExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
