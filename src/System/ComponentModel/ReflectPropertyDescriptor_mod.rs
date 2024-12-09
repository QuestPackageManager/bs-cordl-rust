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
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanResetValue", (component))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderCanResetValue(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExtenderCanResetValue", (provider, component))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderGetReceiverType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ExtenderGetReceiverType", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderGetType(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("ExtenderGetType", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderGetValue(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ExtenderGetValue", (provider, component))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderResetValue(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
        notifyDesc: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtenderResetValue", (provider, component, notifyDesc))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderSetValue(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        notifyDesc: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtenderSetValue", (provider, component, value, notifyDesc))?;
        Ok(__cordl_ret)
    }
    pub fn ExtenderShouldSerializeValue(
        &mut self,
        provider: *mut crate::System::ComponentModel::IExtenderProvider,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExtenderShouldSerializeValue", (provider, component))?;
        Ok(__cordl_ret)
    }
    pub fn FillAttributes(
        &mut self,
        attributes: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttributes", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue(
        &mut self,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetValue", (component))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentClass, name, _cordl_type, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        propInfo: *mut crate::System::Reflection::PropertyInfo,
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn New_Type_MethodInfo_MethodInfo_Il2CppArray2(
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        receiverType: *mut crate::System::Type,
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn OnValueChanged(
        &mut self,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
        e: *mut crate::System::EventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValueChanged", (component, e))?;
        Ok(__cordl_ret)
    }
    pub fn ResetValue(
        &mut self,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetValue", (component))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue(
        &mut self,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (component, value))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldSerializeValue(
        &mut self,
        component: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerializeValue", (component))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentClass, name, _cordl_type, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PropertyInfo_MethodInfo_MethodInfo_Il2CppArray1(
        &mut self,
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        propInfo: *mut crate::System::Reflection::PropertyInfo,
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_MethodInfo_MethodInfo_Il2CppArray2(
        &mut self,
        componentClass: *mut crate::System::Type,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        receiverType: *mut crate::System::Type,
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        attrs: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
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
        Ok(__cordl_ret)
    }
    pub fn get_AmbientValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_AmbientValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ComponentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ComponentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_DefaultValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_GetMethodValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsExtender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsExtender", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PropertyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_PropertyType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_ResetMethodValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SetMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_SetMethodValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldSerializeMethodValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_ShouldSerializeMethodValue", ())?;
        Ok(__cordl_ret)
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
