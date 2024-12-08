#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MeansImplicitUseAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _UseKindFlags_k__BackingField: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    pub _TargetFlags_k__BackingField: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::MeansImplicitUseAttribute => "JetBrains.Annotations"
    ."MeansImplicitUseAttribute"
);
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    pub fn get_TargetFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::JetBrains::Annotations::ImplicitUseTargetFlags = __cordl_object
            .invoke("get_TargetFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UseKindFlags(
        &mut self,
        value: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseKindFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TargetFlags(
        &mut self,
        value: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_UseKindFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::JetBrains::Annotations::ImplicitUseKindFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::JetBrains::Annotations::ImplicitUseKindFlags = __cordl_object
            .invoke("get_UseKindFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ImplicitUseKindFlags1(
        &mut self,
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useKindFlags))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ImplicitUseTargetFlags2(
        &mut self,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (targetFlags))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ImplicitUseKindFlags_ImplicitUseTargetFlags3(
        &mut self,
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useKindFlags, targetFlags))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ImplicitUseKindFlags1(
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useKindFlags))?;
        Ok(__cordl_object)
    }
    pub fn New_ImplicitUseTargetFlags2(
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (targetFlags))?;
        Ok(__cordl_object)
    }
    pub fn New_ImplicitUseKindFlags_ImplicitUseTargetFlags3(
        useKindFlags: crate::JetBrains::Annotations::ImplicitUseKindFlags,
        targetFlags: crate::JetBrains::Annotations::ImplicitUseTargetFlags,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useKindFlags, targetFlags))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "JetBrains+Annotations+MeansImplicitUseAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::MeansImplicitUseAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
