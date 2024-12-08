#[cfg(feature = "UnityEngine+GameObject")]
#[repr(C)]
#[derive(Debug)]
pub struct GameObject {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+GameObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GameObject => "UnityEngine"
    ."GameObject"
);
#[cfg(feature = "UnityEngine+GameObject")]
impl std::ops::Deref for crate::UnityEngine::GameObject {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GameObject")]
impl std::ops::DerefMut for crate::UnityEngine::GameObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GameObject")]
impl crate::UnityEngine::GameObject {
    pub fn GetComponentInParent_Type__cordl_bool0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentInParent", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInParent_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentInParent", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInParent_2<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetComponentInParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInParent__cordl_bool3<T>(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetComponentInParent", (includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetComponent_ByRefMut0<T>(
        &mut self,
        component: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetComponent", (component))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetComponent_Type_ByRefMut1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        component: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetComponent", (_cordl_type, component))?;
        Ok(__cordl_ret)
    }
    pub fn AddComponent_Type0(
        &mut self,
        componentType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("AddComponent", (componentType))?;
        Ok(__cordl_ret)
    }
    pub fn AddComponent_1<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("AddComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInChildren_Type__cordl_bool0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentInChildren", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInChildren_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentInChildren", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInChildren_2<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetComponentInChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentInChildren__cordl_bool3<T>(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetComponentInChildren", (includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetComponentCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isStatic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isStatic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentFastPath(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        oneFurtherThanResultValue: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponentFastPath", (_cordl_type, oneFurtherThanResultValue))?;
        Ok(__cordl_ret)
    }
    pub fn set_tag(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tag", (value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetComponentFastPath(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        oneFurtherThanResultValue: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TryGetComponentFastPath",
                (_cordl_type, oneFurtherThanResultValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetComponents_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object.invoke("GetComponents", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponents_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetComponents", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponents_Type_List_1_2(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        results: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Component,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponents", (_cordl_type, results))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponents_List_1_3<T>(
        &mut self,
        results: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponents", (results))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessageUpwards_SendMessageOptions0(
        &mut self,
        methodName: *mut crate::System::String,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessageUpwards", (methodName, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessageUpwards_Object_SendMessageOptions1(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessageUpwards", (methodName, value, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessageUpwards_Object2(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessageUpwards", (methodName, value))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessageUpwards_String3(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessageUpwards", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn set_layer(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_active(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_active", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isStaticBatchable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isStaticBatchable", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentByNameWithCase(
        &mut self,
        _cordl_type: *mut crate::System::String,
        caseSensitive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentByNameWithCase", (_cordl_type, caseSensitive))?;
        Ok(__cordl_ret)
    }
    pub fn SetActive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInternal(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        useSearchTypeAsArrayReturnType: bool,
        recursive: bool,
        includeInactive: bool,
        reverse: bool,
        resultList: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Array> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Array = __cordl_object
            .invoke(
                "GetComponentsInternal",
                (
                    _cordl_type,
                    useSearchTypeAsArrayReturnType,
                    recursive,
                    includeInactive,
                    reverse,
                    resultList,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_active(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_active", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_activeSelf(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_activeSelf", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_gameObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object.invoke("GetComponentsInChildren", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren_Type__cordl_bool1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object
            .invoke("GetComponentsInChildren", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren__cordl_bool2<T>(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetComponentsInChildren", (includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren__cordl_bool_List_1_3<T>(
        &mut self,
        includeInactive: bool,
        results: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponentsInChildren", (includeInactive, results))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren_4<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetComponentsInChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInChildren_List_1_5<T>(
        &mut self,
        results: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponentsInChildren", (results))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_activeInHierarchy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sceneCullingMask(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_sceneCullingMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareTag(
        &mut self,
        tag: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompareTag", (tag))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponent_0<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponent_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponent", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponent_String2(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponent", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray2(
        &mut self,
        name: *mut crate::System::String,
        components: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, components))?;
        Ok(__cordl_ret)
    }
    pub fn get_scene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = __cordl_object
            .invoke("get_scene", ())?;
        Ok(__cordl_ret)
    }
    pub fn BroadcastMessage_SendMessageOptions0(
        &mut self,
        methodName: *mut crate::System::String,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BroadcastMessage", (methodName, options))?;
        Ok(__cordl_ret)
    }
    pub fn BroadcastMessage_Object_SendMessageOptions1(
        &mut self,
        methodName: *mut crate::System::String,
        parameter: *mut crate::System::Object,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BroadcastMessage", (methodName, parameter, options))?;
        Ok(__cordl_ret)
    }
    pub fn BroadcastMessage_Object2(
        &mut self,
        methodName: *mut crate::System::String,
        parameter: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BroadcastMessage", (methodName, parameter))?;
        Ok(__cordl_ret)
    }
    pub fn BroadcastMessage_String3(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BroadcastMessage", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentAtIndex_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentAtIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentAtIndex_i32_1<T>(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetComponentAtIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_layer(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isStatic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isStatic", ())?;
        Ok(__cordl_ret)
    }
    pub fn QueryComponentAtIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("QueryComponentAtIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetComponentInternal(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("TryGetComponentInternal", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentByName(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("GetComponentByName", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_tag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_tag", ())?;
        Ok(__cordl_ret)
    }
    pub fn Internal_AddComponentWithType(
        &mut self,
        componentType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("Internal_AddComponentWithType", (componentType))?;
        Ok(__cordl_ret)
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetActiveRecursively(
        &mut self,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveRecursively", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessage_SendMessageOptions0(
        &mut self,
        methodName: *mut crate::System::String,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessage", (methodName, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessage_Object_SendMessageOptions1(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
        options: crate::UnityEngine::SendMessageOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessage", (methodName, value, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessage_Object2(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessage", (methodName, value))?;
        Ok(__cordl_ret)
    }
    pub fn SendMessage_String3(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessage", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn AddComponentInternal(
        &mut self,
        className: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Component> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Component = __cordl_object
            .invoke("AddComponentInternal", (className))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInParent_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object.invoke("GetComponentsInParent", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInParent_Type__cordl_bool1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object
            .invoke("GetComponentsInParent", (_cordl_type, includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInParent__cordl_bool_List_1_2<T>(
        &mut self,
        includeInactive: bool,
        results: *mut crate::System::Collections::Generic::List_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetComponentsInParent", (includeInactive, results))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInParent__cordl_bool3<T>(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetComponentsInParent", (includeInactive))?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsInParent_4<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("GetComponentsInParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentIndex(
        &mut self,
        component: *mut crate::UnityEngine::Component,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetComponentIndex", (component))?;
        Ok(__cordl_ret)
    }
    pub fn get_scene_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::SceneManagement::Scene>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_scene_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray2(
        name: *mut crate::System::String,
        components: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, components))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+GameObject")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GameObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
