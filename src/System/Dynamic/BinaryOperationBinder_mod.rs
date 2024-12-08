#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryOperationBinder {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObjectBinder,
    pub _Operation_k__BackingField: crate::System::Linq::Expressions::ExpressionType,
}
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::BinaryOperationBinder =>
    "System.Dynamic"."BinaryOperationBinder"
);
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
impl std::ops::Deref for crate::System::Dynamic::BinaryOperationBinder {
    type Target = crate::System::Dynamic::DynamicMetaObjectBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
impl std::ops::DerefMut for crate::System::Dynamic::BinaryOperationBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
impl crate::System::Dynamic::BinaryOperationBinder {
    pub fn get_Operation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::ExpressionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Linq::Expressions::ExpressionType = __cordl_object
            .invoke("get_Operation", ())?;
        Ok(__cordl_ret)
    }
    pub fn Bind(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("Bind", (target, args))?;
        Ok(__cordl_ret)
    }
    pub fn FallbackBinaryOperation_DynamicMetaObject_DynamicMetaObject0(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        arg: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("FallbackBinaryOperation", (target, arg))?;
        Ok(__cordl_ret)
    }
    pub fn FallbackBinaryOperation_DynamicMetaObject1(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        arg: *mut crate::System::Dynamic::DynamicMetaObject,
        errorSuggestion: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("FallbackBinaryOperation", (target, arg, errorSuggestion))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::BinaryOperationBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
