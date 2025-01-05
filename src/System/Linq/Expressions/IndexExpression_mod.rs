#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _arguments: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        >,
    >,
    pub _Object_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
    pub _Indexer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::PropertyInfo,
    >,
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::IndexExpression =>
    "System.Linq.Expressions"."IndexExpression"
);
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::IndexExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::IndexExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl crate::System::Linq::Expressions::IndexExpression {
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ExpressionVisitor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Accept", (visitor))?;
        Ok(__cordl_ret.into())
    }
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
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (instance, indexer, arguments))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        arguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Rewrite", (instance, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        indexer: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        arguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (instance, indexer, arguments))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Indexer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::PropertyInfo,
        > = __cordl_object.invoke("get_Indexer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("get_Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::IndexExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl AsRef<crate::System::Linq::Expressions::IArgumentProvider>
for crate::System::Linq::Expressions::IndexExpression {
    fn as_ref(&self) -> &crate::System::Linq::Expressions::IArgumentProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+IndexExpression")]
impl AsMut<crate::System::Linq::Expressions::IArgumentProvider>
for crate::System::Linq::Expressions::IndexExpression {
    fn as_mut(&mut self) -> &mut crate::System::Linq::Expressions::IArgumentProvider {
        unsafe { std::mem::transmute(self) }
    }
}
