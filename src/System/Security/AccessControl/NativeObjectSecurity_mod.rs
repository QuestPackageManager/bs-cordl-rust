#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeObjectSecurity {
    __cordl_parent: crate::System::Security::AccessControl::CommonObjectSecurity,
    pub exception_from_error_code: *mut crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode,
}
#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::NativeObjectSecurity =>
    "System.Security.AccessControl"."NativeObjectSecurity"
);
#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
impl std::ops::Deref for crate::System::Security::AccessControl::NativeObjectSecurity {
    type Target = crate::System::Security::AccessControl::CommonObjectSecurity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::NativeObjectSecurity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
impl crate::System::Security::AccessControl::NativeObjectSecurity {
    #[cfg(
        feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
    )]
    pub type ExceptionFromErrorCode = crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode;
    pub fn ClearAccessControlSectionsModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAccessControlSectionsModified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGet(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalGet", (name, includeSections))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_AccessControlSections1(
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, resourceType, name, includeSections))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_AccessControlSections_NativeObjectSecurity_ExceptionFromErrorCode_Il2CppObject2(
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
        exceptionFromErrorCode: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode,
        >,
        exceptionContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    isContainer,
                    resourceType,
                    name,
                    includeSections,
                    exceptionFromErrorCode,
                    exceptionContext,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_NativeObjectSecurity_ExceptionFromErrorCode_Il2CppObject0(
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        exceptionFromErrorCode: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode,
        >,
        exceptionContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (isContainer, resourceType, exceptionFromErrorCode, exceptionContext),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RaiseExceptionOnFailure(
        &mut self,
        errorCode: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseExceptionOnFailure", (errorCode, name, handle, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_AccessControlSections1(
        &mut self,
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, resourceType, name, includeSections))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_AccessControlSections_NativeObjectSecurity_ExceptionFromErrorCode_Il2CppObject2(
        &mut self,
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
        exceptionFromErrorCode: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode,
        >,
        exceptionContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    isContainer,
                    resourceType,
                    name,
                    includeSections,
                    exceptionFromErrorCode,
                    exceptionContext,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NativeObjectSecurity_ExceptionFromErrorCode_Il2CppObject0(
        &mut self,
        isContainer: bool,
        resourceType: crate::System::Security::AccessControl::ResourceType,
        exceptionFromErrorCode: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode,
        >,
        exceptionContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (isContainer, resourceType, exceptionFromErrorCode, exceptionContext),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+AccessControl+NativeObjectSecurity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::NativeObjectSecurity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativeObjectSecurity_ExceptionFromErrorCode {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode =>
    "System.Security.AccessControl"."NativeObjectSecurity/ExceptionFromErrorCode"
);
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
impl std::ops::Deref
for crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
impl std::ops::DerefMut
for crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
impl crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode {
    pub fn Invoke(
        &mut self,
        errorCode: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("Invoke", (errorCode, name, handle, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Security+AccessControl+NativeObjectSecurity+ExceptionFromErrorCode"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::NativeObjectSecurity_ExceptionFromErrorCode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
