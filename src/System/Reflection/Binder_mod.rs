#[cfg(feature = "System+Reflection+Binder")]
#[repr(C)]
#[derive(Debug)]
pub struct Binder {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Reflection+Binder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Binder =>
    "System.Reflection"."Binder"
);
#[cfg(feature = "System+Reflection+Binder")]
impl std::ops::Deref for crate::System::Reflection::Binder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Binder")]
impl std::ops::DerefMut for crate::System::Reflection::Binder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Binder")]
impl crate::System::Reflection::Binder {
    pub fn BindToField(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::FieldInfo,
        >,
        value: *mut crate::System::Object,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::FieldInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::FieldInfo = __cordl_object
            .invoke("BindToField", (bindingAttr, _cordl_match, value, culture))?;
        Ok(__cordl_ret)
    }
    pub fn BindToMethod(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MethodBase,
        >,
        args: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        >,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
        culture: *mut crate::System::Globalization::CultureInfo,
        names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        state: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke(
                "BindToMethod",
                (bindingAttr, _cordl_match, args, modifiers, culture, names, state),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType(
        &mut self,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, _cordl_type, culture))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReorderArgumentArray(
        &mut self,
        args: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        >,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReorderArgumentArray", (args, state))?;
        Ok(__cordl_ret)
    }
    pub fn SelectMethod(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MethodBase,
        >,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke("SelectMethod", (bindingAttr, _cordl_match, types, modifiers))?;
        Ok(__cordl_ret)
    }
    pub fn SelectProperty(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::PropertyInfo,
        >,
        returnType: *mut crate::System::Type,
        indexes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::PropertyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::PropertyInfo = __cordl_object
            .invoke(
                "SelectProperty",
                (bindingAttr, _cordl_match, returnType, indexes, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+Binder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::Binder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
