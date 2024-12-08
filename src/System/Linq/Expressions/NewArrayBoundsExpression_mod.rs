#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct NewArrayBoundsExpression {
    __cordl_parent: crate::System::Linq::Expressions::NewArrayExpression,
}
#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::NewArrayBoundsExpression => "System.Linq.Expressions"
    ."NewArrayBoundsExpression"
);
#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::NewArrayBoundsExpression {
    type Target = crate::System::Linq::Expressions::NewArrayExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::NewArrayBoundsExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
impl crate::System::Linq::Expressions::NewArrayBoundsExpression {
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        expressions: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, expressions))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        expressions: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, expressions))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Linq+Expressions+NewArrayBoundsExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::NewArrayBoundsExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
