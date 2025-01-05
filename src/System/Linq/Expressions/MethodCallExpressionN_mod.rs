#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodCallExpressionN {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::MethodCallExpression,
    >,
    pub _arguments: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    >,
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::MethodCallExpressionN
    => "System.Linq.Expressions"."MethodCallExpressionN"
);
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl std::ops::Deref for crate::System::Linq::Expressions::MethodCallExpressionN {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::MethodCallExpression,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MethodCallExpressionN {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl crate::System::Linq::Expressions::MethodCallExpressionN {
    pub fn GetArgument(
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
        > = __cordl_object.invoke("GetArgument", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, args))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = __cordl_object.invoke("Rewrite", (instance, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MethodCallExpressionN {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IArgumentProvider>,
> for crate::System::Linq::Expressions::MethodCallExpressionN {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::IArgumentProvider,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpressionN")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::IArgumentProvider>,
> for crate::System::Linq::Expressions::MethodCallExpressionN {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::IArgumentProvider,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
