#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_InstanceRestriction {
    __cordl_parent: crate::System::Dynamic::BindingRestrictions,
    pub _expression: *mut crate::System::Linq::Expressions::Expression,
    pub _instance: *mut crate::System::Object,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BindingRestrictions_InstanceRestriction => "System.Dynamic"
    ."BindingRestrictions/InstanceRestriction"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
impl std::ops::Deref
for crate::GlobalNamespace::BindingRestrictions_InstanceRestriction {
    type Target = crate::System::Dynamic::BindingRestrictions;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BindingRestrictions_InstanceRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
impl crate::GlobalNamespace::BindingRestrictions_InstanceRestriction {
    pub fn _ctor(
        &mut self,
        parameter: *mut crate::System::Linq::Expressions::Expression,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameter, instance))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn GetExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("GetExpression", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parameter: *mut crate::System::Linq::Expressions::Expression,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameter, instance))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BindingRestrictions_InstanceRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
