#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_AttributeFilterCacheItem {
    __cordl_parent: crate::System::Object,
    pub _filter: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    pub FilteredMembers: *mut crate::System::Collections::ICollection,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem =>
    "System.ComponentModel"."TypeDescriptor/AttributeFilterCacheItem"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
impl crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem {
    pub fn IsValid(
        &mut self,
        filter: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (filter))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filter: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        filteredMembers: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filter, filteredMembers))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        filter: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        filteredMembers: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filter, filteredMembers))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TypeDescriptionNode_DefaultExtendedTypeDescriptor {
    pub _node: *mut crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    pub _instance: *mut crate::System::Object,
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptionNode_DefaultExtendedTypeDescriptor =>
    "System.ComponentModel"
    ."TypeDescriptor/TypeDescriptionNode/DefaultExtendedTypeDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ComponentModel::TypeDescriptionNode_DefaultExtendedTypeDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
impl crate::System::ComponentModel::TypeDescriptionNode_DefaultExtendedTypeDescriptor {
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetAttributes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetClassName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetComponentName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetConverter",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
            (editorBaseType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (attributes),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (attributes),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
            (pd),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (node, instance),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TypeDescriptionNode_DefaultTypeDescriptor {
    pub _node: *mut crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    pub _objectType: *mut crate::System::Type,
    pub _instance: *mut crate::System::Object,
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptionNode_DefaultTypeDescriptor =>
    "System.ComponentModel"."TypeDescriptor/TypeDescriptionNode/DefaultTypeDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ComponentModel::TypeDescriptionNode_DefaultTypeDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
impl crate::System::ComponentModel::TypeDescriptionNode_DefaultTypeDescriptor {
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetAttributes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetClassName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetComponentName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetConverter",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
            (editorBaseType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (attributes),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (attributes),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
            (pd),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        objectType: *mut crate::System::Type,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (node, objectType, instance),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_FilterCacheItem {
    __cordl_parent: crate::System::Object,
    pub _filterService: *mut crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
    pub FilteredMembers: *mut crate::System::Collections::ICollection,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_FilterCacheItem => "System.ComponentModel"
    ."TypeDescriptor/FilterCacheItem"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptor_FilterCacheItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_FilterCacheItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
impl crate::System::ComponentModel::TypeDescriptor_FilterCacheItem {
    pub fn IsValid(
        &mut self,
        filterService: *mut crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (filterService))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        filterService: *mut crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
        filteredMembers: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filterService, filteredMembers))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        filterService: *mut crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
        filteredMembers: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filterService, filteredMembers))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_FilterCacheItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_IUnimplemented {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_IUnimplemented => "System.ComponentModel"
    ."TypeDescriptor/IUnimplemented"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptor_IUnimplemented {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_IUnimplemented {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
impl crate::System::ComponentModel::TypeDescriptor_IUnimplemented {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_IUnimplemented {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_MemberDescriptorComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer =>
    "System.ComponentModel"."TypeDescriptor/MemberDescriptorComparer"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    pub fn Compare(
        &mut self,
        left: *mut crate::System::Object,
        right: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (left, right))?;
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_MergedTypeDescriptor {
    __cordl_parent: crate::System::Object,
    pub _primary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    pub _secondary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor =>
    "System.ComponentModel"."TypeDescriptor/MergedTypeDescriptor"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    pub fn New(
        primary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
        secondary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (primary, secondary))?;
        Ok(__cordl_object)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetClassName", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetComponentName", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetConverter", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
                (editorBaseType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
                (attributes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
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
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
                (attributes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
                (pd),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        primary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
        secondary: *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (primary, secondary))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_TypeDescriptionNode {
    __cordl_parent: crate::System::ComponentModel::TypeDescriptionProvider,
    pub Next: *mut crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    pub Provider: *mut crate::System::ComponentModel::TypeDescriptionProvider,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_TypeDescriptionNode =>
    "System.ComponentModel"."TypeDescriptor/TypeDescriptionNode"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode {
    type Target = crate::System::ComponentModel::TypeDescriptionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
impl crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode {
    #[cfg(
        feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
    )]
    pub type DefaultExtendedTypeDescriptor = crate::System::ComponentModel::TypeDescriptionNode_DefaultExtendedTypeDescriptor;
    #[cfg(
        feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
    )]
    pub type DefaultTypeDescriptor = crate::System::ComponentModel::TypeDescriptionNode_DefaultTypeDescriptor;
    pub fn CreateInstance(
        &mut self,
        provider: *mut crate::System::IServiceProvider,
        objectType: *mut crate::System::Type,
        argTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateInstance", (provider, objectType, argTypes, args))?;
        Ok(__cordl_ret)
    }
    pub fn GetCache(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("GetCache", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtendedTypeDescriptor(
        &mut self,
        instance: *mut crate::System::Object,
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
        instance: *mut crate::System::Object,
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
    pub fn GetReflectionType(
        &mut self,
        objectType: *mut crate::System::Type,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetReflectionType", (objectType, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeDescriptor(
        &mut self,
        objectType: *mut crate::System::Type,
        instance: *mut crate::System::Object,
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
    pub fn New(
        provider: *mut crate::System::ComponentModel::TypeDescriptionProvider,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        provider: *mut crate::System::ComponentModel::TypeDescriptionProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::TypeDescriptor =>
    "System.ComponentModel"."TypeDescriptor"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::TypeDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
impl crate::System::ComponentModel::TypeDescriptor {
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
    pub type TypeDescriptorComObject = crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
    type IUnimplemented = crate::System::ComponentModel::TypeDescriptor_IUnimplemented;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
    pub type AttributeFilterCacheItem = crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
    pub type FilterCacheItem = crate::System::ComponentModel::TypeDescriptor_FilterCacheItem;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
    pub type MemberDescriptorComparer = crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
    pub type MergedTypeDescriptor = crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
    pub type TypeDescriptionNode = crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
    pub type TypeDescriptorInterface = crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface;
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_TypeDescriptorComObject {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject =>
    "System.ComponentModel"."TypeDescriptor/TypeDescriptorComObject"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
impl crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject {}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_TypeDescriptorInterface {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface =>
    "System.ComponentModel"."TypeDescriptor/TypeDescriptorInterface"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
impl crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface {}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
