#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicMetaObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _Expression_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
    pub _Restrictions_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Dynamic::BindingRestrictions,
    >,
}
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::DynamicMetaObject =>
    "System.Dynamic"."DynamicMetaObject"
);
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
impl std::ops::Deref for crate::System::Dynamic::DynamicMetaObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
impl std::ops::DerefMut for crate::System::Dynamic::DynamicMetaObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
impl crate::System::Dynamic::DynamicMetaObject {
    pub fn BindBinaryOperation(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BinaryOperationBinder>,
        arg: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindBinaryOperation", (binder, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindConvert(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ConvertBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindConvert", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindCreateInstance(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::CreateInstanceBinder>,
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
        > = __cordl_object.invoke("BindCreateInstance", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindDeleteIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DeleteIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("BindDeleteIndex", (binder, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindDeleteMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DeleteMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindDeleteMember", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindGetIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("BindGetIndex", (binder, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindGetMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindGetMember", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindInvoke(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeBinder>,
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
        > = __cordl_object.invoke("BindInvoke", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindInvokeMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeMemberBinder>,
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
        > = __cordl_object.invoke("BindInvokeMember", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindSetIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindSetIndex", (binder, indexes, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindSetMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetMemberBinder>,
        value: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindSetMember", (binder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindUnaryOperation(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::UnaryOperationBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindUnaryOperation", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (value, expression))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDynamicMemberNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("GetDynamicMemberNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Expression_BindingRestrictions0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, restrictions))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject1(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, restrictions, value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Expression_BindingRestrictions0(
        &mut self,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, restrictions))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, restrictions, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Expression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("get_Expression", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LimitType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_LimitType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Restrictions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = __cordl_object.invoke("get_Restrictions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RuntimeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_RuntimeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::DynamicMetaObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
