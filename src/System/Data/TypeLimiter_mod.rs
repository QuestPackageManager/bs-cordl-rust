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
    pub fn Capture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Capture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTypeIsAllowed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        capturedLimiter: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureTypeIsAllowed", (_cordl_type, capturedLimiter))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterRestrictedScope_DataSet0(
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnterRestrictedScope", (dataSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnterRestrictedScope_DataTable1(
        dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnterRestrictedScope", (dataTable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviouslyDeclaredDataTypes_DataSet1(
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviouslyDeclaredDataTypes", (dataSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviouslyDeclaredDataTypes_DataTable0(
        dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPreviouslyDeclaredDataTypes", (dataTable))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scope))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        scope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsTypeLimitingDisabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsTypeLimitingDisabled", ())?;
        Ok(__cordl_ret.into())
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAllowedType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAllowedType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeUnconditionallyAllowed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTypeUnconditionallyAllowed", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        previousScope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
        allowedTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (previousScope, allowedTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        previousScope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
        allowedTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (previousScope, allowedTypes))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl AsRef<crate::System::IDisposable> for crate::System::Data::TypeLimiter_Scope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl AsMut<crate::System::IDisposable> for crate::System::Data::TypeLimiter_Scope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
