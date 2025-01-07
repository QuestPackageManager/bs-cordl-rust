#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryOperationBinder {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObjectBinder,
    pub _Operation_k__BackingField: crate::System::Linq::Expressions::ExpressionType,
}
#[cfg(feature = "System+Dynamic+BinaryOperationBinder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::BinaryOperationBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "BinaryOperationBinder";
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
    pub fn Bind(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("Bind", (target, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn FallbackBinaryOperation_DynamicMetaObject1(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        arg: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object
            .invoke("FallbackBinaryOperation", (target, arg, errorSuggestion))?;
        Ok(__cordl_ret.into())
    }
    pub fn FallbackBinaryOperation_DynamicMetaObject_DynamicMetaObject0(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        arg: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("FallbackBinaryOperation", (target, arg))?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
