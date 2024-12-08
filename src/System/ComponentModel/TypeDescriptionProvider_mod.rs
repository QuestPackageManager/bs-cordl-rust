#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptionProvider_EmptyCustomTypeDescriptor {
    __cordl_parent: crate::System::ComponentModel::CustomTypeDescriptor,
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor =>
    "System.ComponentModel"."TypeDescriptionProvider/EmptyCustomTypeDescriptor"
);
#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
impl std::ops::Deref
for crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor {
    type Target = crate::System::ComponentModel::CustomTypeDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
impl std::ops::DerefMut
for crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
impl crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDescriptionProvider {
    __cordl_parent: crate::System::Object,
    pub _parent: *mut crate::System::ComponentModel::TypeDescriptionProvider,
    pub _emptyDescriptor: *mut crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor,
}
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::TypeDescriptionProvider
    => "System.ComponentModel"."TypeDescriptionProvider"
);
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
impl std::ops::Deref for crate::System::ComponentModel::TypeDescriptionProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
impl std::ops::DerefMut for crate::System::ComponentModel::TypeDescriptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
impl crate::System::ComponentModel::TypeDescriptionProvider {
    #[cfg(
        feature = "System+ComponentModel+TypeDescriptionProvider+EmptyCustomTypeDescriptor"
    )]
    pub type EmptyCustomTypeDescriptor = crate::System::ComponentModel::TypeDescriptionProvider_EmptyCustomTypeDescriptor;
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
    pub fn GetReflectionType_Type0(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetReflectionType", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn GetReflectionType_Object1(
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
    pub fn GetTypeDescriptor_Type0(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ICustomTypeDescriptor = __cordl_object
            .invoke("GetTypeDescriptor", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeDescriptor_Object1(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::ICustomTypeDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ICustomTypeDescriptor = __cordl_object
            .invoke("GetTypeDescriptor", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeDescriptor_Type_Object2(
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+ComponentModel+TypeDescriptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::TypeDescriptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
