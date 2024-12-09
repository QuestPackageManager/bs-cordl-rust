#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonAcl {
    __cordl_parent: crate::System::Security::AccessControl::GenericAcl,
    pub is_aefa: bool,
    pub is_canonical: bool,
    pub is_container: bool,
    pub is_ds: bool,
    pub raw_acl: *mut crate::System::Security::AccessControl::RawAcl,
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::CommonAcl =>
    "System.Security.AccessControl"."CommonAcl"
);
#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
impl std::ops::Deref for crate::System::Security::AccessControl::CommonAcl {
    type Target = crate::System::Security::AccessControl::GenericAcl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::CommonAcl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
impl crate::System::Security::AccessControl::CommonAcl {
    #[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
    pub type RemoveAcesCallback_1<T: quest_hook::libil2cpp::Type> = crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<
        T,
    >;
    pub fn AddAceGetQualifiedAce(
        &mut self,
        aceQualifier: crate::System::Security::AccessControl::AceQualifier,
        sid: *mut crate::System::Security::Principal::SecurityIdentifier,
        accessMask: i32,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        auditFlags: crate::System::Security::AccessControl::AuditFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::QualifiedAce,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::QualifiedAce = __cordl_object
            .invoke(
                "AddAceGetQualifiedAce",
                (
                    aceQualifier,
                    sid,
                    accessMask,
                    inheritanceFlags,
                    propagationFlags,
                    auditFlags,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddAce_AceQualifier_SecurityIdentifier_i32_InheritanceFlags_PropagationFlags_AuditFlags0(
        &mut self,
        aceQualifier: crate::System::Security::AccessControl::AceQualifier,
        sid: *mut crate::System::Security::Principal::SecurityIdentifier,
        accessMask: i32,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        auditFlags: crate::System::Security::AccessControl::AuditFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddAce",
                (
                    aceQualifier,
                    sid,
                    accessMask,
                    inheritanceFlags,
                    propagationFlags,
                    auditFlags,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddAce_QualifiedAce1(
        &mut self,
        newAce: *mut crate::System::Security::AccessControl::QualifiedAce,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAce", (newAce))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyCanonicalSortToExplicitAces_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyCanonicalSortToExplicitAces", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyCanonicalSortToExplicitAces_i32_i32_1(
        &mut self,
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyCanonicalSortToExplicitAces", (start, count))?;
        Ok(__cordl_ret)
    }
    pub fn CanonicalizeAndClearAefa(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CanonicalizeAndClearAefa", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAceFlags(
        &mut self,
        inheritanceFlags: crate::System::Security::AccessControl::InheritanceFlags,
        propagationFlags: crate::System::Security::AccessControl::PropagationFlags,
        auditFlags: crate::System::Security::AccessControl::AuditFlags,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::AccessControl::AceFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::AccessControl::AceFlags = __cordl_object
            .invoke("GetAceFlags", (inheritanceFlags, propagationFlags, auditFlags))?;
        Ok(__cordl_ret)
    }
    pub fn GetAceInsertPosition(
        &mut self,
        aceQualifier: crate::System::Security::AccessControl::AceQualifier,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetAceInsertPosition", (aceQualifier))?;
        Ok(__cordl_ret)
    }
    pub fn GetCanonicalExplicitAceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCanonicalExplicitAceCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCanonicalExplicitDenyAceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCanonicalExplicitDenyAceCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        isContainer: bool,
        isDS: bool,
        rawAcl: *mut crate::System::Security::AccessControl::RawAcl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (isContainer, isDS, rawAcl))?;
        Ok(__cordl_ret)
    }
    pub fn IsAceMeaningless(
        &mut self,
        ace: *mut crate::System::Security::AccessControl::GenericAce,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAceMeaningless", (ace))?;
        Ok(__cordl_ret)
    }
    pub fn MergeExplicitAcePair(
        &mut self,
        ace1: *mut crate::System::Security::AccessControl::GenericAce,
        ace2: *mut crate::System::Security::AccessControl::GenericAce,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::GenericAce,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::GenericAce = __cordl_object
            .invoke("MergeExplicitAcePair", (ace1, ace2))?;
        Ok(__cordl_ret)
    }
    pub fn MergeExplicitAces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeExplicitAces", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_1(
        isContainer: bool,
        isDS: bool,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, isDS, capacity))?;
        Ok(__cordl_object)
    }
    pub fn New_u8_i32_0(
        isContainer: bool,
        isDS: bool,
        revision: u8,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isContainer, isDS, revision, capacity))?;
        Ok(__cordl_object)
    }
    pub fn RemoveAces<T>(
        &mut self,
        callback: *mut crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAces", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn RequireCanonicity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequireCanonicity", ())?;
        Ok(__cordl_ret)
    }
    pub fn TestCanonicity(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TestCanonicity", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        isContainer: bool,
        isDS: bool,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, isDS, capacity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_i32_0(
        &mut self,
        isContainer: bool,
        isDS: bool,
        revision: u8,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isContainer, isDS, revision, capacity))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCanonical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCanonical", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDS(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDS", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::AccessControl::GenericAce,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::AccessControl::GenericAce = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsAefa(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsAefa", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: *mut crate::System::Security::AccessControl::GenericAce,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (index, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::CommonAcl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonAcl_RemoveAcesCallback_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1 < T > =>
    "System.Security.AccessControl"."CommonAcl/RemoveAcesCallback`1" < T >
);
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<T> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<T> {
    pub fn Invoke(&mut self, ace: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (ace))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAcl+RemoveAcesCallback_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Security::AccessControl::CommonAcl_RemoveAcesCallback_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
