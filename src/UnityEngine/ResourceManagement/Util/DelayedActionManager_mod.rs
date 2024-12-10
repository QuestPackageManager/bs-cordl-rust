#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct DelayedActionManager {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::UnityEngine::ResourceManagement::Util::DelayedActionManager,
    >,
    pub m_Actions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
        >,
    >,
    pub m_DelayedActions: *mut crate::System::Collections::Generic::LinkedList_1<
        crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
    >,
    pub m_NodeCache: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
        >,
    >,
    pub m_CollectionIndex: i32,
    pub m_DestroyOnCompletion: bool,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::DelayedActionManager =>
    "UnityEngine.ResourceManagement.Util"."DelayedActionManager"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::DelayedActionManager {
    type Target = crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::UnityEngine::ResourceManagement::Util::DelayedActionManager,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::DelayedActionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
impl crate::UnityEngine::ResourceManagement::Util::DelayedActionManager {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager+DelegateInfo"
    )]
    pub type DelegateInfo = crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo;
    pub fn AddActionInternal(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        delay: f32,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddActionInternal", (action, delay, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyWhenComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyWhenComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNode(
        &mut self,
        del: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo,
            >,
        > = __cordl_object.invoke("GetNode", (del))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalLateUpdate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalLateUpdate", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::DelayedActionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager+DelegateInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DelayedActionManager_DelegateInfo {
    pub m_Id: i32,
    pub m_Delegate: *mut crate::System::Delegate,
    pub m_Target: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _InvocationTime_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager+DelegateInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo =>
    "UnityEngine.ResourceManagement.Util"."DelayedActionManager/DelegateInfo"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager+DelegateInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DelayedActionManager+DelegateInfo")]
impl crate::UnityEngine::ResourceManagement::Util::DelayedActionManager_DelegateInfo {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Invoke",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        invocationTime: f32,
        p: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (d, invocationTime, p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InvocationTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InvocationTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InvocationTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_InvocationTime",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
