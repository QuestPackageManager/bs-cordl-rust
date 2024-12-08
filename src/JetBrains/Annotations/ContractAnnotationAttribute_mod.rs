#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ContractAnnotationAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Contract_k__BackingField: *mut crate::System::String,
    pub _ForceFullStates_k__BackingField: bool,
}
#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::ContractAnnotationAttribute => "JetBrains.Annotations"
    ."ContractAnnotationAttribute"
);
#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::ContractAnnotationAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::ContractAnnotationAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
impl crate::JetBrains::Annotations::ContractAnnotationAttribute {
    pub fn _ctor_String0(
        &mut self,
        contract: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contract))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        contract: *mut crate::System::String,
        forceFullStates: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contract, forceFullStates))?;
        Ok(__cordl_ret)
    }
    pub fn set_ForceFullStates(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ForceFullStates", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Contract(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Contract", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Contract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Contract", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ForceFullStates(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ForceFullStates", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        contract: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contract))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        contract: *mut crate::System::String,
        forceFullStates: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contract, forceFullStates))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "JetBrains+Annotations+ContractAnnotationAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::ContractAnnotationAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
