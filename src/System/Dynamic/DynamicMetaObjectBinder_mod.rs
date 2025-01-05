#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicMetaObjectBinder {
    __cordl_parent: crate::System::Runtime::CompilerServices::CallSiteBinder,
}
#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::DynamicMetaObjectBinder =>
    "System.Dynamic"."DynamicMetaObjectBinder"
);
#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
impl std::ops::Deref for crate::System::Dynamic::DynamicMetaObjectBinder {
    type Target = crate::System::Runtime::CompilerServices::CallSiteBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
impl std::ops::DerefMut for crate::System::Dynamic::DynamicMetaObjectBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
impl crate::System::Dynamic::DynamicMetaObjectBinder {
    pub fn Bind_DynamicMetaObject_Il2CppArray1(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
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
    pub fn Bind_Il2CppArray_ReadOnlyCollection_1_LabelTarget0(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
        returnLabel: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::LabelTarget,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("Bind", (args, parameters, returnLabel))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgumentMetaObjects(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::Expressions::ParameterExpression,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgumentMetaObjects", (args, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUpdateExpression(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetUpdateExpression", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsStandardBinder(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsStandardBinder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ReturnType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObjectBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::DynamicMetaObjectBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
