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
    pub fn BindToField(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::FieldInfo>,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cultureInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::FieldInfo,
        > = __cordl_object
            .invoke("BindToField", (bindingAttr, _cordl_match, value, cultureInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindToMethod(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodBase,
            >,
        >,
        args: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Reflection::ParameterModifier,
            >,
        >,
        cultureInfo: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        >,
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        state: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = __cordl_object
            .invoke(
                "BindToMethod",
                (bindingAttr, _cordl_match, args, modifiers, cultureInfo, names, state),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CanChangePrimitive(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanChangePrimitive", (source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanConvertPrimitive(
        source: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        target: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanConvertPrimitive", (source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanConvertPrimitiveObjectToType(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanConvertPrimitiveObjectToType", (source, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanPrimitiveWiden(
        source: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanPrimitiveWiden", (source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        cultureInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ChangeType", (value, _cordl_type, cultureInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareMethodSig(
        m1: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        m2: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareMethodSig", (m1, m2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareMethodSigAndName(
        m1: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        m2: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareMethodSigAndName", (m1, m2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateParamOrder(
        paramOrder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        pars: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateParamOrder", (paramOrder, pars, names))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExactBinding(
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodBase,
            >,
        >,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Reflection::ParameterModifier,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExactBinding", (_cordl_match, types, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExactPropertyBinding(
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
        returnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Reflection::ParameterModifier,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::PropertyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExactPropertyBinding",
                (_cordl_match, returnType, types, modifiers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostDerivedNewSlotMeth(
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodBase,
            >,
        >,
        cMatches: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMostDerivedNewSlotMeth", (_cordl_match, cMatches))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostSpecific(
        p1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
        paramOrder1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        paramArrayType1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        p2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ParameterInfo,
            >,
        >,
        paramOrder2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        paramArrayType2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindMostSpecific",
                (
                    p1,
                    paramOrder1,
                    paramArrayType1,
                    p2,
                    paramOrder2,
                    paramArrayType2,
                    types,
                    args,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostSpecificField(
        cur1: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        cur2: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMostSpecificField", (cur1, cur2))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostSpecificMethod(
        m1: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        paramOrder1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        paramArrayType1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        m2: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        paramOrder2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        paramArrayType2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindMostSpecificMethod",
                (
                    m1,
                    paramOrder1,
                    paramArrayType1,
                    m2,
                    paramOrder2,
                    paramArrayType2,
                    types,
                    args,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostSpecificProperty(
        cur1: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        cur2: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMostSpecificProperty", (cur1, cur2))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMostSpecificType(
        c1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        c2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMostSpecificType", (c1, c2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHierarchyDepth(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHierarchyDepth", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReorderArgumentArray(
        &mut self,
        args: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReorderArgumentArray", (args, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReorderParams(
        paramOrder: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        vars: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReorderParams", (paramOrder, vars))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectMethod(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodBase,
            >,
        >,
        types: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Reflection::ParameterModifier,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = __cordl_object
            .invoke("SelectMethod", (bindingAttr, _cordl_match, types, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectProperty(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        _cordl_match: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
        returnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Reflection::ParameterModifier,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::PropertyInfo,
        > = __cordl_object
            .invoke(
                "SelectProperty",
                (bindingAttr, _cordl_match, returnType, indexes, modifiers),
            )?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_argsMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        argsMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        originalSize: i32,
        isParamArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (argsMap, originalSize, isParamArray))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        argsMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        originalSize: i32,
        isParamArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (argsMap, originalSize, isParamArray))?;
        Ok(__cordl_ret.into())
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
