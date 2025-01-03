#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
#[repr(C)]
#[derive(Debug)]
pub struct FullExpression_1<TDelegate: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Expressions::ExpressionN_1<TDelegate>,
    pub _NameCore_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _TailCallCore_k__BackingField: bool,
    __cordl_phantom_TDelegate: std::marker::PhantomData<TDelegate>,
}
#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::FullExpression_1 <
    TDelegate > => "System.Linq.Expressions"."FullExpression`1" < TDelegate >
);
#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Expressions::FullExpression_1<TDelegate> {
    type Target = crate::System::Linq::Expressions::ExpressionN_1<TDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Expressions::FullExpression_1<TDelegate> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
impl<
    TDelegate: quest_hook::libil2cpp::Type,
> crate::System::Linq::Expressions::FullExpression_1<TDelegate> {
    pub fn New(
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (body, name, tailCall, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        body: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tailCall: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Linq::Expressions::ParameterExpression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (body, name, tailCall, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_NameCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TailCallCore(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TailCallCore", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+FullExpression_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::FullExpression_1<TDelegate> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
