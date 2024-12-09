#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor {
    __cordl_parent: crate::System::ComponentModel::PropertyDescriptor,
    pub target: *mut quest_hook::libil2cpp::Il2CppObject,
    pub property: *mut crate::System::ComponentModel::PropertyDescriptor,
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor
    => "System.ComponentModel.Design"
    ."DesignerOptionService/DesignerOptionCollection/WrappedPropertyDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
impl std::ops::Deref
for crate::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor {
    type Target = crate::System::ComponentModel::PropertyDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
impl crate::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor {
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
    pub fn New(
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        target: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (property, target))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        target: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (property, target))?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("get_Attributes", ())?;
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
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor {
    __cordl_parent: crate::System::ComponentModel::PropertyDescriptor,
    pub _option: *mut crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection,
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor
    => "System.ComponentModel.Design"
    ."DesignerOptionService/DesignerOptionConverter/OptionPropertyDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
impl std::ops::Deref
for crate::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor {
    type Target = crate::System::ComponentModel::PropertyDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
impl crate::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor {
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
    pub fn New(
        option: *mut crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (option))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        option: *mut crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (option))?;
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
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerOptionService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::DesignerOptionService =>
    "System.ComponentModel.Design"."DesignerOptionService"
);
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
impl std::ops::Deref for crate::System::ComponentModel::Design::DesignerOptionService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::DesignerOptionService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
impl crate::System::ComponentModel::Design::DesignerOptionService {
    #[cfg(
        feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
    )]
    pub type DesignerOptionCollection = crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection;
    #[cfg(
        feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
    )]
    pub type DesignerOptionConverter = crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter;
    pub fn PopulateOptionCollection(
        &mut self,
        options: *mut crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateOptionCollection", (options))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+Design+DesignerOptionService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::DesignerOptionService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerOptionService_DesignerOptionCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _service: *mut crate::System::ComponentModel::Design::DesignerOptionService,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _value: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _children: *mut crate::System::Collections::ArrayList,
    pub _properties: *mut crate::System::ComponentModel::PropertyDescriptorCollection,
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection =>
    "System.ComponentModel.Design"."DesignerOptionService/DesignerOptionCollection"
);
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
impl std::ops::Deref
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
impl crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection {
    #[cfg(
        feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection+WrappedPropertyDescriptor"
    )]
    pub type WrappedPropertyDescriptor = crate::System::ComponentModel::Design::DesignerOptionCollection_DesignerOptionService_WrappedPropertyDescriptor;
    pub fn CopyTo(
        &mut self,
        array: *mut crate::System::Array,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn EnsurePopulated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsurePopulated", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionCollection"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerOptionService_DesignerOptionConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter =>
    "System.ComponentModel.Design"."DesignerOptionService/DesignerOptionConverter"
);
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
impl std::ops::Deref
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
impl crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter {
    #[cfg(
        feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter+OptionPropertyDescriptor"
    )]
    pub type OptionPropertyDescriptor = crate::System::ComponentModel::Design::DesignerOptionConverter_DesignerOptionService_OptionPropertyDescriptor;
    pub fn ConvertTo(
        &mut self,
        cxt: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        culture: *mut crate::System::Globalization::CultureInfo,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ConvertTo", (cxt, culture, value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
        cxt: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetProperties", (cxt, value, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertiesSupported(
        &mut self,
        cxt: *mut crate::System::ComponentModel::ITypeDescriptorContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetPropertiesSupported", (cxt))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(
    feature = "System+ComponentModel+Design+DesignerOptionService+DesignerOptionConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::DesignerOptionService_DesignerOptionConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
