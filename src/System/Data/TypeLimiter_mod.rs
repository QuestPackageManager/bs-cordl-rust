#[cfg(feature = "System+Data+TypeLimiter")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeLimiter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_instanceScope: *mut crate::System::Data::TypeLimiter_Scope,
}
#[cfg(feature = "System+Data+TypeLimiter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::TypeLimiter => "System.Data"
    ."TypeLimiter"
);
#[cfg(feature = "System+Data+TypeLimiter")]
impl std::ops::Deref for crate::System::Data::TypeLimiter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl std::ops::DerefMut for crate::System::Data::TypeLimiter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl crate::System::Data::TypeLimiter {
    #[cfg(feature = "System+Data+TypeLimiter+Scope")]
    pub type Scope = crate::System::Data::TypeLimiter_Scope;
    #[cfg(feature = "System+Data+TypeLimiter+__c")]
    pub type __c = crate::System::Data::TypeLimiter___c;
    pub fn New(
        scope: *mut crate::System::Data::TypeLimiter_Scope,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scope))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        scope: *mut crate::System::Data::TypeLimiter_Scope,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scope))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::TypeLimiter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeLimiter_Scope {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_allowedTypes: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::Type,
    >,
    pub m_previousScope: *mut crate::System::Data::TypeLimiter_Scope,
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::TypeLimiter_Scope => "System.Data"
    ."TypeLimiter/Scope"
);
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl std::ops::Deref for crate::System::Data::TypeLimiter_Scope {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl std::ops::DerefMut for crate::System::Data::TypeLimiter_Scope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl crate::System::Data::TypeLimiter_Scope {
    #[cfg(feature = "System+Data+TypeLimiter+Scope+__c")]
    pub type __c = crate::System::Data::Scope_TypeLimiter___c;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsAllowedType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAllowedType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        previousScope: *mut crate::System::Data::TypeLimiter_Scope,
        allowedTypes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (previousScope, allowedTypes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        previousScope: *mut crate::System::Data::TypeLimiter_Scope,
        allowedTypes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (previousScope, allowedTypes))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::TypeLimiter_Scope {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
