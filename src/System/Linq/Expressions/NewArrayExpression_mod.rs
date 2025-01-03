#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct NewArrayExpression {
    __cordl_parent: crate::System::Linq::Expressions::Expression,
    pub _Type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _Expressions_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::NewArrayExpression =>
    "System.Linq.Expressions"."NewArrayExpression"
);
#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::NewArrayExpression {
    type Target = crate::System::Linq::Expressions::Expression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::NewArrayExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
impl crate::System::Linq::Expressions::NewArrayExpression {
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
    pub fn Make(
        nodeType: crate::System::Linq::Expressions::ExpressionType,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewArrayExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Make", (nodeType, _cordl_type, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, expressions))?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::NewArrayExpression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::NewArrayExpression,
        > = __cordl_object.invoke("Update", (expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        expressions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, expressions))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Expressions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        > = __cordl_object.invoke("get_Expressions", ())?;
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
#[cfg(feature = "System+Linq+Expressions+NewArrayExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::NewArrayExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
