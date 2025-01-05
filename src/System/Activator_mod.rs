#[cfg(feature = "System+Activator")]
#[repr(C)]
#[derive(Debug)]
pub struct Activator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Activator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Activator => "System"."Activator"
);
#[cfg(feature = "System+Activator")]
impl std::ops::Deref for crate::System::Activator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Activator")]
impl std::ops::DerefMut for crate::System::Activator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Activator")]
impl crate::System::Activator {
    pub fn CreateInstance_7<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type4(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type_BindingFlags_Binder_Il2CppArray_CultureInfo0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
        binder: quest_hook::libil2cpp::Gc<crate::System::Reflection::Binder>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInstance",
                (_cordl_type, bindingAttr, binder, args, culture),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type_BindingFlags_Binder_Il2CppArray_CultureInfo_Il2CppArray1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
        binder: quest_hook::libil2cpp::Gc<crate::System::Reflection::Binder>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInstance",
                (_cordl_type, bindingAttr, binder, args, culture, activationAttributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type_Il2CppArray2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type_Il2CppArray_Il2CppArray3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type, args, activationAttributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type__cordl_bool5(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type, nonPublic))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance_Type__cordl_bool__cordl_bool6(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
        wrapExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstance", (_cordl_type, nonPublic, wrapExceptions))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Activator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Activator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
