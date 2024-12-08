#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtenderProvidedPropertyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ExtenderProperty_k__BackingField: *mut crate::System::ComponentModel::PropertyDescriptor,
    pub _Provider_k__BackingField: *mut crate::System::ComponentModel::IExtenderProvider,
    pub _ReceiverType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::ExtenderProvidedPropertyAttribute =>
    "System.ComponentModel"."ExtenderProvidedPropertyAttribute"
);
#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
impl std::ops::Deref
for crate::System::ComponentModel::ExtenderProvidedPropertyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
impl std::ops::DerefMut
for crate::System::ComponentModel::ExtenderProvidedPropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
impl crate::System::ComponentModel::ExtenderProvidedPropertyAttribute {
    pub fn get_ReceiverType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ReceiverType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ReceiverType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReceiverType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Provider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::IExtenderProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::IExtenderProvider = __cordl_object
            .invoke("get_Provider", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn IsDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDefaultAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExtenderProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("get_ExtenderProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ExtenderProperty(
        &mut self,
        value: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExtenderProperty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Provider(
        &mut self,
        value: *mut crate::System::ComponentModel::IExtenderProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Provider", (value))?;
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
#[cfg(feature = "System+ComponentModel+ExtenderProvidedPropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ExtenderProvidedPropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
