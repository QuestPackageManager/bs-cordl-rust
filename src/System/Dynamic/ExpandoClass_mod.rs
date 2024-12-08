#[cfg(feature = "System+Dynamic+ExpandoClass")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpandoClass {
    __cordl_parent: crate::System::Object,
    pub _keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _hashCode: i32,
    pub _transitions: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::WeakReference,
        >,
    >,
}
#[cfg(feature = "System+Dynamic+ExpandoClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::ExpandoClass =>
    "System.Dynamic"."ExpandoClass"
);
#[cfg(feature = "System+Dynamic+ExpandoClass")]
impl std::ops::Deref for crate::System::Dynamic::ExpandoClass {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoClass")]
impl std::ops::DerefMut for crate::System::Dynamic::ExpandoClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+ExpandoClass")]
impl crate::System::Dynamic::ExpandoClass {
    pub fn FindNewClass(
        &mut self,
        newKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::ExpandoClass> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::ExpandoClass = __cordl_object
            .invoke("FindNewClass", (newKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetTransitionList(
        &mut self,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::WeakReference,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::WeakReference,
        > = __cordl_object.invoke("GetTransitionList", (hashCode))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueIndex(
        &mut self,
        name: *mut crate::System::String,
        caseInsensitive: bool,
        obj: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetValueIndex", (name, caseInsensitive, obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueIndexCaseInsensitive(
        &mut self,
        name: *mut crate::System::String,
        obj: *mut crate::System::Dynamic::ExpandoObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetValueIndexCaseInsensitive", (name, obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueIndexCaseSensitive(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetValueIndexCaseSensitive", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_1(
        keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keys, hashCode))?;
        Ok(__cordl_object)
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
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        keys: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keys, hashCode))?;
        Ok(__cordl_ret)
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Dynamic+ExpandoClass")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::ExpandoClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
