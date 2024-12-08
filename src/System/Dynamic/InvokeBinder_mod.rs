#[cfg(feature = "System+Dynamic+InvokeBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct InvokeBinder {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObjectBinder,
}
#[cfg(feature = "System+Dynamic+InvokeBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::InvokeBinder =>
    "System.Dynamic"."InvokeBinder"
);
#[cfg(feature = "System+Dynamic+InvokeBinder")]
impl std::ops::Deref for crate::System::Dynamic::InvokeBinder {
    type Target = crate::System::Dynamic::DynamicMetaObjectBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+InvokeBinder")]
impl std::ops::DerefMut for crate::System::Dynamic::InvokeBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+InvokeBinder")]
impl crate::System::Dynamic::InvokeBinder {
    pub fn FallbackInvoke_DynamicMetaObject_Il2CppArray0(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("FallbackInvoke", (target, args))?;
        Ok(__cordl_ret)
    }
    pub fn FallbackInvoke_DynamicMetaObject1(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
        errorSuggestion: *mut crate::System::Dynamic::DynamicMetaObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("FallbackInvoke", (target, args, errorSuggestion))?;
        Ok(__cordl_ret)
    }
    pub fn Bind(
        &mut self,
        target: *mut crate::System::Dynamic::DynamicMetaObject,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("Bind", (target, args))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+InvokeBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::InvokeBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
