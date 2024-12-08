#[cfg(feature = "UnityEngine+MonoBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoBehaviour {
    __cordl_parent: crate::UnityEngine::Behaviour,
    pub m_CancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
}
#[cfg(feature = "UnityEngine+MonoBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MonoBehaviour => "UnityEngine"
    ."MonoBehaviour"
);
#[cfg(feature = "UnityEngine+MonoBehaviour")]
impl std::ops::Deref for crate::UnityEngine::MonoBehaviour {
    type Target = crate::UnityEngine::Behaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+MonoBehaviour")]
impl std::ops::DerefMut for crate::UnityEngine::MonoBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+MonoBehaviour")]
impl crate::UnityEngine::MonoBehaviour {
    pub fn StopCoroutineFromEnumeratorManaged(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutineFromEnumeratorManaged", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StopAllCoroutines(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAllCoroutines", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsInvoking_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInvoking", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsInvoking_String1(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInvoking", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeRepeating(
        &mut self,
        methodName: *mut crate::System::String,
        _cordl_time: f32,
        repeatRate: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeRepeating", (methodName, _cordl_time, repeatRate))?;
        Ok(__cordl_ret)
    }
    pub fn CancelInvoke_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelInvoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn CancelInvoke_String1(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelInvoke", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        methodName: *mut crate::System::String,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (methodName, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutineManaged2(
        &mut self,
        enumerator: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutineManaged2", (enumerator))?;
        Ok(__cordl_ret)
    }
    pub fn StopCoroutineManaged(
        &mut self,
        routine: *mut crate::UnityEngine::Coroutine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutineManaged", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn get_destroyCancellationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationToken = __cordl_object
            .invoke("get_destroyCancellationToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutine_String0(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutine", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutine_String_Object1(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutine", (methodName, value))?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutine_IEnumerator2(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutineManaged(
        &mut self,
        methodName: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutineManaged", (methodName, value))?;
        Ok(__cordl_ret)
    }
    pub fn StartCoroutine_Auto(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutine_Auto", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StopCoroutine_IEnumerator0(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StopCoroutine_Coroutine1(
        &mut self,
        routine: *mut crate::UnityEngine::Coroutine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StopCoroutine_String2(
        &mut self,
        methodName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutine", (methodName))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseCancellation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseCancellation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_useGUILayout(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useGUILayout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_useGUILayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useGUILayout", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetScriptClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetScriptClassName", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnCancellationTokenCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancellationTokenCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+MonoBehaviour")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::MonoBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
