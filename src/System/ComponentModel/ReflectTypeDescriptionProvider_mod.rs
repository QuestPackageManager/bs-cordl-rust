#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectTypeDescriptionProvider {
    __cordl_parent: crate::System::ComponentModel::TypeDescriptionProvider,
    pub _typeData: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ReflectTypeDescriptionProvider => "System.ComponentModel"
    ."ReflectTypeDescriptionProvider"
);
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    type Target = crate::System::ComponentModel::TypeDescriptionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl std::ops::DerefMut
for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    #[cfg(
        feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
    )]
    pub type ReflectedTypeData = crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData;
    pub fn CreateInstance(
        &mut self,
        provider: *mut crate::System::IServiceProvider,
        objectType: *mut crate::System::Type,
        argTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CreateInstance", (provider, objectType, argTypes, args))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributes(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("GetAttributes", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetCache(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("GetCache", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetClassName(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetClassName", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentName(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetComponentName", (_cordl_type, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetConverter(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = __cordl_object
            .invoke("GetConverter", (_cordl_type, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultEvent(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = __cordl_object
            .invoke("GetDefaultEvent", (_cordl_type, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultProperty(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("GetDefaultProperty", (_cordl_type, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetEditor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetEditor", (_cordl_type, instance, editorBaseType))?;
        Ok(__cordl_ret)
    }
    pub fn GetEvents(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke("GetEvents", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedAttributes(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("GetExtendedAttributes", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedClassName(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetExtendedClassName", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedComponentName(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetExtendedComponentName", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedConverter(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = __cordl_object
            .invoke("GetExtendedConverter", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedDefaultEvent(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = __cordl_object
            .invoke("GetExtendedDefaultEvent", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedDefaultProperty(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("GetExtendedDefaultProperty", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedEditor(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetExtendedEditor", (instance, editorBaseType))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedEvents(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke("GetExtendedEvents", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedProperties(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetExtendedProperties", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedPropertyOwner(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetExtendedPropertyOwner", (instance, pd))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedTypeDescriptor(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ICustomTypeDescriptor = __cordl_object
            .invoke("GetExtendedTypeDescriptor", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtenderProviders(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::IExtenderProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::IExtenderProvider,
        > = __cordl_object.invoke("GetExtenderProviders", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetProperties", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyOwner(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetPropertyOwner", (_cordl_type, instance, pd))?;
        Ok(__cordl_ret)
    }
    pub fn GetReflectionType(
        &mut self,
        objectType: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetReflectionType", (objectType, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeData(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        createIfNeeded: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData = __cordl_object
            .invoke("GetTypeData", (_cordl_type, createIfNeeded))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeDescriptor(
        &mut self,
        objectType: *mut crate::System::Type,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ICustomTypeDescriptor = __cordl_object
            .invoke("GetTypeDescriptor", (objectType, instance))?;
        Ok(__cordl_ret)
    }
    pub fn IsPopulated(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPopulated", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Refresh(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (_cordl_type))?;
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
#[cfg(feature = "System+ComponentModel+ReflectTypeDescriptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectTypeDescriptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectTypeDescriptionProvider_ReflectedTypeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type: *mut crate::System::Type,
    pub _attributes: *mut crate::System::ComponentModel::AttributeCollection,
    pub _events: *mut crate::System::ComponentModel::EventDescriptorCollection,
    pub _properties: *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    pub _converter: *mut crate::System::ComponentModel::TypeConverter,
    pub _editors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _editorTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub _editorCount: i32,
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData =>
    "System.ComponentModel"."ReflectTypeDescriptionProvider/ReflectedTypeData"
);
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl std::ops::Deref
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    pub fn GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("GetAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClassName(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetClassName", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentName(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetComponentName", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetConverter(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = __cordl_object
            .invoke("GetConverter", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultEvent(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = __cordl_object
            .invoke("GetDefaultEvent", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultProperty(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("GetDefaultProperty", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetEditor(
        &mut self,
        instance: *mut quest_hook::libil2cpp::Il2CppObject,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetEditor", (instance, editorBaseType))?;
        Ok(__cordl_ret)
    }
    pub fn GetEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke("GetEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeFromName(
        &mut self,
        typeName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetTypeFromName", (typeName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPopulated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPopulated", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+ComponentModel+ReflectTypeDescriptionProvider+ReflectedTypeData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectTypeDescriptionProvider_ReflectedTypeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
