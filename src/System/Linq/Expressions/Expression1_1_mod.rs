#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Expression1_1<TDelegate: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Linq::Expressions::Expression_1<TDelegate>,
    pub _par0: *mut crate::System::Object,
    __cordl_phantom_TDelegate: std::marker::PhantomData<TDelegate>,
}
#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Expression1_1 <
    TDelegate > => "System.Linq.Expressions"."Expression1`1" < TDelegate >
);
#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Expressions::Expression1_1<TDelegate> {
    type Target = crate::System::Linq::Expressions::Expression_1<TDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Expressions::Expression1_1<TDelegate> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
impl<
    TDelegate: quest_hook::libil2cpp::Type,
> crate::System::Linq::Expressions::Expression1_1<TDelegate> {
    pub fn GetParameter(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::ParameterExpression,
    >
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::ParameterExpression = __cordl_object
            .invoke("GetParameter", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        body: *mut crate::System::Linq::Expressions::Expression,
        par0: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (body, par0))?;
        Ok(__cordl_object)
    }
    pub fn Rewrite(
        &mut self,
        body: *mut crate::System::Linq::Expressions::Expression,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Linq::Expressions::ParameterExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression_1<TDelegate>,
    >
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression_1<
            TDelegate,
        > = __cordl_object.invoke("Rewrite", (body, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        body: *mut crate::System::Linq::Expressions::Expression,
        par0: *mut crate::System::Linq::Expressions::ParameterExpression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (body, par0))?;
        Ok(__cordl_ret)
    }
    pub fn get_ParameterCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ParameterCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Expression1_1")]
impl<TDelegate: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Expression1_1<TDelegate> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
