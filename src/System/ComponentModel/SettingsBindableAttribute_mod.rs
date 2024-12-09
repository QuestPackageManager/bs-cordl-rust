#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsBindableAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Bindable_k__BackingField: bool,
}
#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::SettingsBindableAttribute => "System.ComponentModel"
    ."SettingsBindableAttribute"
);
#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::SettingsBindableAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::SettingsBindableAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
impl crate::System::ComponentModel::SettingsBindableAttribute {
    pub fn Equals(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(bindable: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindable))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bindable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindable))?;
        Ok(__cordl_ret)
    }
    pub fn get_Bindable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Bindable", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ComponentModel+SettingsBindableAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::SettingsBindableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
