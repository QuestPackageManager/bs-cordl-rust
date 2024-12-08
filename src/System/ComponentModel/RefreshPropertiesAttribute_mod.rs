#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RefreshPropertiesAttribute {
    __cordl_parent: crate::System::Attribute,
    pub refresh: crate::System::ComponentModel::RefreshProperties,
}
#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::RefreshPropertiesAttribute => "System.ComponentModel"
    ."RefreshPropertiesAttribute"
);
#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::RefreshPropertiesAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::RefreshPropertiesAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
impl crate::System::ComponentModel::RefreshPropertiesAttribute {
    pub fn Equals(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDefaultAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        refresh: crate::System::ComponentModel::RefreshProperties,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (refresh))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        refresh: crate::System::ComponentModel::RefreshProperties,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (refresh))?;
        Ok(__cordl_ret)
    }
    pub fn get_RefreshProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ComponentModel::RefreshProperties,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::RefreshProperties = __cordl_object
            .invoke("get_RefreshProperties", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+RefreshPropertiesAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::RefreshPropertiesAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
