#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectPropertyDescriptor {
    __cordl_parent: crate::System::ComponentModel::PropertyDescriptor,
    pub state: crate::System::Collections::Specialized::BitVector32,
    pub componentClass: *mut crate::System::Type,
    pub _cordl_type: *mut crate::System::Type,
    pub defaultValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub ambientValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub propInfo: *mut crate::System::Reflection::PropertyInfo,
    pub getMethod: *mut crate::System::Reflection::MethodInfo,
    pub setMethod: *mut crate::System::Reflection::MethodInfo,
    pub shouldSerializeMethod: *mut crate::System::Reflection::MethodInfo,
    pub resetMethod: *mut crate::System::Reflection::MethodInfo,
    pub realChangedEvent: *mut crate::System::ComponentModel::EventDescriptor,
    pub receiverType: *mut crate::System::Type,
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ReflectPropertyDescriptor => "System.ComponentModel"
    ."ReflectPropertyDescriptor"
);
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectPropertyDescriptor {
    type Target = crate::System::ComponentModel::PropertyDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::ReflectPropertyDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl crate::System::ComponentModel::ReflectPropertyDescriptor {
    pub fn CanResetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanResetValue", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderCanResetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExtenderCanResetValue", (provider, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetReceiverType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("ExtenderGetReceiverType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetType(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("ExtenderGetType", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderGetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ExtenderGetValue", (provider, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderResetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        notifyDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtenderResetValue", (provider, component, notifyDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderSetValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        notifyDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtenderSetValue", (provider, component, value, notifyDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtenderShouldSerializeValue(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IExtenderProvider,
        >,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExtenderShouldSerializeValue", (provider, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentClass, name, _cordl_type, attributes))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        propInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    propInfo,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_MethodInfo_MethodInfo_Il2CppArray2(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        receiverType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    receiverType,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OnValueChanged(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValueChanged", (component, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetValue", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (component, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerializeValue(
        &mut self,
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerializeValue", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentClass, name, _cordl_type, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        propInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    propInfo,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_MethodInfo_MethodInfo_Il2CppArray2(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        receiverType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attrs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    componentClass,
                    name,
                    _cordl_type,
                    receiverType,
                    getMethod,
                    setMethod,
                    attrs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AmbientValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AmbientValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ComponentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ComponentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_DefaultValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_GetMethodValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExtender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsExtender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_PropertyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_ResetMethodValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_SetMethodValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldSerializeMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_ShouldSerializeMethodValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ReflectPropertyDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectPropertyDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
