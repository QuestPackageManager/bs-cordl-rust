#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicMetaObject {
    __cordl_parent: crate::System::Object,
    pub _value: *mut crate::System::Object,
    pub _Expression_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _Restrictions_k__BackingField: *mut crate::System::Dynamic::BindingRestrictions,
}
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::DynamicMetaObject =>
    "System.Dynamic"."DynamicMetaObject"
);
#[cfg(feature = "System+Dynamic+DynamicMetaObject")]
impl std::ops::Deref for crate::System::Dynamic::DynamicMetaObject {
    type Target = crate::System::Object;
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
        binder: *mut crate::System::Dynamic::BinaryOperationBinder,
        arg: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindBinaryOperation", (binder, arg))?;
        Ok(__cordl_ret)
    }
    pub fn BindGetIndex(
        &mut self,
        binder: *mut crate::System::Dynamic::GetIndexBinder,
        indexes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindGetIndex", (binder, indexes))?;
        Ok(__cordl_ret)
    }
    pub fn get_LimitType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_LimitType", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindDeleteMember(
        &mut self,
        binder: *mut crate::System::Dynamic::DeleteMemberBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindDeleteMember", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn BindSetIndex(
        &mut self,
        binder: *mut crate::System::Dynamic::SetIndexBinder,
        indexes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
        value: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindSetIndex", (binder, indexes, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Expression_BindingRestrictions0(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, restrictions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object1(
        &mut self,
        expression: *mut crate::System::Linq::Expressions::Expression,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, restrictions, value))?;
        Ok(__cordl_ret)
    }
    pub fn BindCreateInstance(
        &mut self,
        binder: *mut crate::System::Dynamic::CreateInstanceBinder,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindCreateInstance", (binder, args))?;
        Ok(__cordl_ret)
    }
    pub fn BindDeleteIndex(
        &mut self,
        binder: *mut crate::System::Dynamic::DeleteIndexBinder,
        indexes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindDeleteIndex", (binder, indexes))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDynamicMemberNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDynamicMemberNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindUnaryOperation(
        &mut self,
        binder: *mut crate::System::Dynamic::UnaryOperationBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindUnaryOperation", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn BindSetMember(
        &mut self,
        binder: *mut crate::System::Dynamic::SetMemberBinder,
        value: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindSetMember", (binder, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Restrictions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Dynamic::BindingRestrictions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::BindingRestrictions = __cordl_object
            .invoke("get_Restrictions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindConvert(
        &mut self,
        binder: *mut crate::System::Dynamic::ConvertBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindConvert", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn get_RuntimeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_RuntimeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindInvokeMember(
        &mut self,
        binder: *mut crate::System::Dynamic::InvokeMemberBinder,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindInvokeMember", (binder, args))?;
        Ok(__cordl_ret)
    }
    pub fn get_Expression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Expression", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindGetMember(
        &mut self,
        binder: *mut crate::System::Dynamic::GetMemberBinder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindGetMember", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn BindInvoke(
        &mut self,
        binder: *mut crate::System::Dynamic::InvokeBinder,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("BindInvoke", (binder, args))?;
        Ok(__cordl_ret)
    }
    pub fn New_Expression_BindingRestrictions0(
        expression: *mut crate::System::Linq::Expressions::Expression,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, restrictions))?;
        Ok(__cordl_object)
    }
    pub fn New_Object1(
        expression: *mut crate::System::Linq::Expressions::Expression,
        restrictions: *mut crate::System::Dynamic::BindingRestrictions,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, restrictions, value))?;
        Ok(__cordl_object)
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
