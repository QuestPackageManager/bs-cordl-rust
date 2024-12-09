#[cfg(feature = "System+DefaultBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultBinder {
    __cordl_parent: crate::System::Reflection::Binder,
}
#[cfg(feature = "System+DefaultBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DefaultBinder => "System"
    ."DefaultBinder"
);
#[cfg(feature = "System+DefaultBinder")]
impl std::ops::Deref for crate::System::DefaultBinder {
    type Target = crate::System::Reflection::Binder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DefaultBinder")]
impl std::ops::DerefMut for crate::System::DefaultBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DefaultBinder")]
impl crate::System::DefaultBinder {
    #[cfg(feature = "System+DefaultBinder+BinderState")]
    pub type BinderState = crate::System::DefaultBinder_BinderState;
    #[cfg(feature = "System+DefaultBinder+Primitives")]
    pub type Primitives = crate::System::DefaultBinder_Primitives;
    #[cfg(feature = "System+DefaultBinder+__c")]
    pub type __c = crate::System::DefaultBinder___c;
    pub fn BindToField(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::FieldInfo,
        >,
        value: *mut crate::System::Object,
        cultureInfo: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::FieldInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::FieldInfo = __cordl_object
            .invoke("BindToField", (bindingAttr, _cordl_match, value, cultureInfo))?;
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
        cultureInfo: *mut crate::System::Globalization::CultureInfo,
        names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        state: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke(
                "BindToMethod",
                (bindingAttr, _cordl_match, args, modifiers, cultureInfo, names, state),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType(
        &mut self,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
        cultureInfo: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, _cordl_type, cultureInfo))?;
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
#[cfg(feature = "System+DefaultBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DefaultBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DefaultBinder+BinderState")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultBinder_BinderState {
    __cordl_parent: crate::System::Object,
    pub m_argsMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_originalSize: i32,
    pub m_isParamArray: bool,
}
#[cfg(feature = "System+DefaultBinder+BinderState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DefaultBinder_BinderState => "System"
    ."DefaultBinder/BinderState"
);
#[cfg(feature = "System+DefaultBinder+BinderState")]
impl std::ops::Deref for crate::System::DefaultBinder_BinderState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DefaultBinder+BinderState")]
impl std::ops::DerefMut for crate::System::DefaultBinder_BinderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DefaultBinder+BinderState")]
impl crate::System::DefaultBinder_BinderState {
    pub fn New(
        argsMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        originalSize: i32,
        isParamArray: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (argsMap, originalSize, isParamArray))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        argsMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        originalSize: i32,
        isParamArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (argsMap, originalSize, isParamArray))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+DefaultBinder+BinderState")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DefaultBinder_BinderState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DefaultBinder+Primitives")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultBinder_Primitives {
    Boolean = 8i32,
    Byte = 64i32,
    Char = 16i32,
    DateTime = 65536i32,
    Decimal = 32768i32,
    Double = 16384i32,
    Int16 = 128i32,
    Int32 = 512i32,
    Int64 = 2048i32,
    SByte = 32i32,
    Single = 8192i32,
    String = 262144i32,
    UInt16 = 256i32,
    UInt32 = 1024i32,
    UInt64 = 4096i32,
}
#[cfg(feature = "System+DefaultBinder+Primitives")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DefaultBinder_Primitives => "System"
    ."DefaultBinder/Primitives"
);
