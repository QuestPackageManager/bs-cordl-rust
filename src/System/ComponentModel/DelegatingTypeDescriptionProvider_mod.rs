#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegatingTypeDescriptionProvider {
    __cordl_parent: crate::System::ComponentModel::TypeDescriptionProvider,
    pub _type: *mut crate::System::Type,
}
#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::DelegatingTypeDescriptionProvider =>
    "System.ComponentModel"."DelegatingTypeDescriptionProvider"
);
#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
impl std::ops::Deref
for crate::System::ComponentModel::DelegatingTypeDescriptionProvider {
    type Target = crate::System::ComponentModel::TypeDescriptionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
impl std::ops::DerefMut
for crate::System::ComponentModel::DelegatingTypeDescriptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
impl crate::System::ComponentModel::DelegatingTypeDescriptionProvider {
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
    pub fn New(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
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
    pub fn get_Provider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeDescriptionProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeDescriptionProvider = __cordl_object
            .invoke("get_Provider", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+DelegatingTypeDescriptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::DelegatingTypeDescriptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
