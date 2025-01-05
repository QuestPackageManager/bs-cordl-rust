#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_MergedRestriction {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Dynamic::BindingRestrictions,
    >,
    pub Left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    pub Right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BindingRestrictions_MergedRestriction => "System.Dynamic"
    ."BindingRestrictions/MergedRestriction"
);
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl std::ops::Deref for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    pub fn GetExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetExpression", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
        right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        left: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
        right: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BindingRestrictions_MergedRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
