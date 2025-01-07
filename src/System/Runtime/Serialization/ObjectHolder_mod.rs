#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_id: i64,
    pub m_missingElementsRemaining: i32,
    pub m_missingDecendents: i32,
    pub m_serInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationInfo,
    >,
    pub m_surrogate: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISerializationSurrogate,
    >,
    pub m_missingElements: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::FixupHolderList,
    >,
    pub m_dependentObjects: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::LongList,
    >,
    pub m_next: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ObjectHolder,
    >,
    pub m_flags: i32,
    pub m_markForFixupWhenAvailable: bool,
    pub m_valueFixup: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ValueTypeFixupInfo,
    >,
    pub m_typeLoad: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::TypeLoadExceptionHolder,
    >,
    pub m_reachable: bool,
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::ObjectHolder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "ObjectHolder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
impl std::ops::Deref for crate::System::Runtime::Serialization::ObjectHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::ObjectHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
impl crate::System::Runtime::Serialization::ObjectHolder {
    pub fn AddDependency(
        &mut self,
        dependentObject: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDependency", (dependentObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFixup(
        &mut self,
        fixup: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::FixupHolder,
        >,
        manager: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFixup", (fixup, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecrementFixupsRemaining(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecrementFixupsRemaining", (manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementDescendentFixups(
        &mut self,
        amount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementDescendentFixups", (amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkForCompletionWhenAvailable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkForCompletionWhenAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppObject_i64_SerializationInfo_ISerializationSurrogate_i64_FieldInfo_Il2CppArray1(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objID: i64,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        surrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
        idOfContainingObj: i64,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        arrayIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (obj, objID, info, surrogate, idOfContainingObj, field, arrayIndex),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_i64_SerializationInfo_ISerializationSurrogate_i64_FieldInfo_Il2CppArray2(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objID: i64,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        surrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
        idOfContainingObj: i64,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        arrayIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (obj, objID, info, surrogate, idOfContainingObj, field, arrayIndex),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i64_0(
        objID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objID))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveDependency(
        &mut self,
        id: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDependency", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectValue(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        manager: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObjectValue", (obj, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        surrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
        idOfContainer: i64,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        arrayIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        manager: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateData",
                (obj, info, surrogate, idOfContainer, field, arrayIndex, manager),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDescendentDependencyChain(
        &mut self,
        amount: i32,
        manager: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ObjectManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDescendentDependencyChain", (amount, manager))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_i64_SerializationInfo_ISerializationSurrogate_i64_FieldInfo_Il2CppArray1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objID: i64,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        surrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
        idOfContainingObj: i64,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        arrayIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (obj, objID, info, surrogate, idOfContainingObj, field, arrayIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_i64_SerializationInfo_ISerializationSurrogate_i64_FieldInfo_Il2CppArray2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objID: i64,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        surrogate: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
        idOfContainingObj: i64,
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        arrayIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (obj, objID, info, surrogate, idOfContainingObj, field, arrayIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_0(
        &mut self,
        objID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objID))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanObjectValueChange(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanObjectValueChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanSurrogatedObjectValueChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CanSurrogatedObjectValueChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletelyFixed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CompletelyFixed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContainerID(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContainerID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DependentObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::LongList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::LongList,
        > = __cordl_object.invoke("get_DependentObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DirectlyDependentObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DirectlyDependentObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasISerializable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasISerializable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasSurrogate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSurrogate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsIncompleteObjectReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsIncompleteObjectReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ObjectValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Reachable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Reachable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresDelayedFixup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresDelayedFixup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresSerInfoFixup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresSerInfoFixup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresValueTypeFixup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresValueTypeFixup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SerializationInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        > = __cordl_object.invoke("get_SerializationInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Surrogate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        > = __cordl_object.invoke("get_Surrogate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalDependentObjects(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TotalDependentObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeLoadException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::TypeLoadExceptionHolder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::TypeLoadExceptionHolder,
        > = __cordl_object.invoke("get_TypeLoadException", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeLoadExceptionReachable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeLoadExceptionReachable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueFixup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ValueTypeFixupInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ValueTypeFixupInfo,
        > = __cordl_object.invoke("get_ValueFixup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueTypeFixupPerformed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ValueTypeFixupPerformed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DependentObjects(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::LongList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DependentObjects", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsIncompleteObjectReference(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsIncompleteObjectReference", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Reachable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Reachable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RequiresSerInfoFixup(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RequiresSerInfoFixup", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SerializationInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SerializationInfo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TypeLoadException(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::TypeLoadExceptionHolder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeLoadException", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValueTypeFixupPerformed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValueTypeFixupPerformed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+ObjectHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ObjectHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
