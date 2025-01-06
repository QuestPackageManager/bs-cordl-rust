#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor {
    pub _node: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    >,
    pub _instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor
    => "System.ComponentModel"
    ."TypeDescriptor/TypeDescriptionNode/DefaultExtendedTypeDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor {
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
impl crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor {
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetAttributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetClassName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetComponentName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetConverter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
            (editorBaseType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (attributes),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (attributes),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
            (pd),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (node, instance),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
impl AsRef<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor {
    fn as_ref(&self) -> &crate::System::ComponentModel::ICustomTypeDescriptor {
        todo!()
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultExtendedTypeDescriptor"
)]
impl AsMut<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor {
    fn as_mut(&mut self) -> &mut crate::System::ComponentModel::ICustomTypeDescriptor {
        todo!()
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor {
    pub _node: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    >,
    pub _objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor =>
    "System.ComponentModel"."TypeDescriptor/TypeDescriptionNode/DefaultTypeDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor {
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
impl crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor {
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetAttributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetClassName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetComponentName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetConverter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
            (editorBaseType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
            (attributes),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
            (attributes),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
            (pd),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (node, objectType, instance),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
impl AsRef<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor {
    fn as_ref(&self) -> &crate::System::ComponentModel::ICustomTypeDescriptor {
        todo!()
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
)]
impl AsMut<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor {
    fn as_mut(&mut self) -> &mut crate::System::ComponentModel::ICustomTypeDescriptor {
        todo!()
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::TypeDescriptor =>
    "System.ComponentModel"."TypeDescriptor"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
    pub type AttributeFilterCacheItem = crate::System::ComponentModel::TypeDescriptor_AttributeFilterCacheItem;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
    pub type FilterCacheItem = crate::System::ComponentModel::TypeDescriptor_FilterCacheItem;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+IUnimplemented")]
    type IUnimplemented = crate::System::ComponentModel::TypeDescriptor_IUnimplemented;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
    pub type MemberDescriptorComparer = crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
    pub type MergedTypeDescriptor = crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
    pub type TypeDescriptionNode = crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
    pub type TypeDescriptorComObject = crate::System::ComponentModel::TypeDescriptor_TypeDescriptorComObject;
    #[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorInterface")]
    pub type TypeDescriptorInterface = crate::System::ComponentModel::TypeDescriptor_TypeDescriptorInterface;
    pub fn AddProvider(
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptionProvider,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddProvider", (provider, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDefaultProvider(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckDefaultProvider", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance(
        provider: quest_hook::libil2cpp::Gc<crate::System::IServiceProvider>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (provider, objectType, argTypes, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterMembers(
        members: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterMembers", (members, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssociation(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        primary: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssociation", (_cordl_type, primary))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes_Il2CppObject1(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes_Il2CppObject__cordl_bool2(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noCustomTypeDesc: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (component, noCustomTypeDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes_Type0(
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (componentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCache(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCache", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConverter(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConverter", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptor_Il2CppObject__cordl_bool1(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noCustomTypeDesc: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDescriptor", (component, noCustomTypeDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptor_Type_Il2CppString0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDescriptor", (_cordl_type, typeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEvents_Il2CppObject1(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEvents", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEvents_Il2CppObject_Il2CppArray__cordl_bool2(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        noCustomTypeDesc: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEvents", (component, attributes, noCustomTypeDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEvents_Type0(
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEvents", (componentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedDescriptor(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExtendedDescriptor", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtenderCollisionSuffix(
        member: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::MemberDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExtenderCollisionSuffix", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeForBaseType(
        searchType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeForBaseType", (searchType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertiesImpl(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        noCustomTypeDesc: bool,
        noAttributes: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetPropertiesImpl",
                (component, attributes, noCustomTypeDesc, noAttributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_Il2CppObject1(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_Il2CppObject_Il2CppArray3(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (component, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_Il2CppObject_Il2CppArray__cordl_bool4(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        noCustomTypeDesc: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (component, attributes, noCustomTypeDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_Il2CppObject__cordl_bool2(
        component: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noCustomTypeDesc: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (component, noCustomTypeDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_Type0(
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (componentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProviderRecursive(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeDescriptionProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptionProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProviderRecursive", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReflectionType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn NodeFor_Il2CppObject2(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NodeFor", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn NodeFor_Il2CppObject__cordl_bool3(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        createDelegator: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NodeFor", (instance, createDelegator))?;
        Ok(__cordl_ret.into())
    }
    pub fn NodeFor_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NodeFor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn NodeFor_Type__cordl_bool1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        createDelegator: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NodeFor", (_cordl_type, createDelegator))?;
        Ok(__cordl_ret.into())
    }
    pub fn PipelineAttributeFilter(
        pipelineType: i32,
        members: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        filter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cache: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PipelineAttributeFilter",
                (pipelineType, members, filter, instance, cache),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PipelineFilter(
        pipelineType: i32,
        members: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cache: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PipelineFilter", (pipelineType, members, instance, cache))?;
        Ok(__cordl_ret.into())
    }
    pub fn PipelineInitialize(
        pipelineType: i32,
        members: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        cache: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PipelineInitialize", (pipelineType, members, cache))?;
        Ok(__cordl_ret.into())
    }
    pub fn PipelineMerge(
        pipelineType: i32,
        primary: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        secondary: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cache: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PipelineMerge",
                (pipelineType, primary, secondary, instance, cache),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRefresh(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaiseRefresh", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Refresh", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldHideMember(
        member: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::MemberDescriptor,
        >,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldHideMember", (member, attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortDescriptorArray(
        infos: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortDescriptorArray", (infos))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ComObjectType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ComObjectType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InterfaceType() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InterfaceType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MetadataVersion() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MetadataVersion", ())?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+AttributeFilterCacheItem")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_AttributeFilterCacheItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _filter: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Attribute>,
        >,
    >,
    pub FilteredMembers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ICollection,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        filter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        filteredMembers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filter, filteredMembers))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
        filteredMembers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filter, filteredMembers))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_FilterCacheItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _filterService: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
    >,
    pub FilteredMembers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ICollection,
    >,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptor_FilterCacheItem => "System.ComponentModel"
    ."TypeDescriptor/FilterCacheItem"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptor+FilterCacheItem")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptor_FilterCacheItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        filterService: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (filterService))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        filterService: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
        >,
        filteredMembers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filterService, filteredMembers))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        filterService: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::Design::ITypeDescriptorFilterService,
        >,
        filteredMembers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filterService, filteredMembers))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl AsRef<crate::System::Collections::IComparer>
for crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MemberDescriptorComparer")]
impl AsMut<crate::System::Collections::IComparer>
for crate::System::ComponentModel::TypeDescriptor_MemberDescriptorComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_MergedTypeDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _primary: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::ICustomTypeDescriptor,
    >,
    pub _secondary: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::ICustomTypeDescriptor,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        primary: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        >,
        secondary: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (primary, secondary))?;
        Ok(__cordl_object.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::AttributeCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::AttributeCollection,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetClassName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetComponentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeConverter,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetConverter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::EventDescriptor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptor,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        > = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
                (editorBaseType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventDescriptorCollection,
        > = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
                (attributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
                (attributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::PropertyDescriptor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
                (pd),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        primary: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        >,
        secondary: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (primary, secondary))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl AsRef<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    fn as_ref(&self) -> &crate::System::ComponentModel::ICustomTypeDescriptor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+MergedTypeDescriptor")]
impl AsMut<crate::System::ComponentModel::ICustomTypeDescriptor>
for crate::System::ComponentModel::TypeDescriptor_MergedTypeDescriptor {
    fn as_mut(&mut self) -> &mut crate::System::ComponentModel::ICustomTypeDescriptor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_TypeDescriptionNode {
    __cordl_parent: crate::System::ComponentModel::TypeDescriptionProvider,
    pub Next: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeDescriptor_TypeDescriptionNode,
    >,
    pub Provider: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::TypeDescriptionProvider,
    >,
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
    pub type DefaultExtendedTypeDescriptor = crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultExtendedTypeDescriptor;
    #[cfg(
        feature = "System+ComponentModel+TypeDescriptor+TypeDescriptionNode+DefaultTypeDescriptor"
    )]
    pub type DefaultTypeDescriptor = crate::System::ComponentModel::TypeDescriptionNode_TypeDescriptor_DefaultTypeDescriptor;
    pub fn CreateInstance(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IServiceProvider>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("CreateInstance", (provider, objectType, argTypes, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCache(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = __cordl_object.invoke("GetCache", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtendedTypeDescriptor(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = __cordl_object.invoke("GetExtendedTypeDescriptor", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtenderProviders(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::IExtenderProvider,
                >,
            >,
        > = __cordl_object.invoke("GetExtenderProviders", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionType(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetReflectionType", (objectType, instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeDescriptor(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ICustomTypeDescriptor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ICustomTypeDescriptor,
        > = __cordl_object.invoke("GetTypeDescriptor", (objectType, instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptionProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::TypeDescriptionProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+ComponentModel+TypeDescriptor+TypeDescriptorComObject")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptor_TypeDescriptorComObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
