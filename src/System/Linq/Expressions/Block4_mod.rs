#[cfg(feature = "System+Linq+Expressions+Block4")]
#[repr(C)]
#[derive(Debug)]
pub struct Block4 {
    __cordl_parent: crate::System::Linq::Expressions::BlockExpression,
    pub _arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    pub _arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    pub _arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
}
#[cfg(feature = "System+Linq+Expressions+Block4")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Linq::Expressions::Block4 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "Block4";
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
#[cfg(feature = "System+Linq+Expressions+Block4")]
impl std::ops::Deref for crate::System::Linq::Expressions::Block4 {
    type Target = crate::System::Linq::Expressions::BlockExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Block4")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Block4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Block4")]
impl crate::System::Linq::Expressions::Block4 {
    pub fn GetExpression(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetExpression", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrMakeExpressions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        > = __cordl_object.invoke("GetOrMakeExpressions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arg0, arg1, arg2, arg3))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        variables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::BlockExpression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::BlockExpression,
        > = __cordl_object.invoke("Rewrite", (variables, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg1: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg2: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        arg3: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arg0, arg1, arg2, arg3))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExpressionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ExpressionCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Block4")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Block4 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
