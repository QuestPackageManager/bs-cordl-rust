#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BoolMonitor_OVRHandTest_BoolGenerator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator => ""
    ."OVRHandTest/BoolMonitor/BoolGenerator"
);
#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
impl crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator {
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHandTest")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHandTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub uiText: *mut crate::UnityEngine::UI::Text,
    pub monitors: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRHandTest_BoolMonitor,
    >,
    pub data: *mut crate::System::Text::StringBuilder,
    pub hs_LH: crate::GlobalNamespace::OVRPlugin_HandState,
    pub hs_RH: crate::GlobalNamespace::OVRPlugin_HandState,
    pub skel_LH: crate::GlobalNamespace::OVRPlugin_Skeleton,
    pub skel_RH: crate::GlobalNamespace::OVRPlugin_Skeleton,
    pub mesh_LH: *mut crate::GlobalNamespace::OVRPlugin_Mesh,
    pub mesh_RH: *mut crate::GlobalNamespace::OVRPlugin_Mesh,
    pub result_skel_LH: bool,
    pub result_skel_RH: bool,
    pub result_mesh_LH: bool,
    pub result_mesh_RH: bool,
}
#[cfg(feature = "OVRHandTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHandTest => ""."OVRHandTest"
);
#[cfg(feature = "OVRHandTest")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHandTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHandTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest")]
impl crate::GlobalNamespace::OVRHandTest {
    #[cfg(feature = "OVRHandTest+BoolMonitor")]
    pub type BoolMonitor = crate::GlobalNamespace::OVRHandTest_BoolMonitor;
    #[cfg(feature = "OVRHandTest+__c")]
    pub type __c = crate::GlobalNamespace::OVRHandTest___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "OVRHandTest")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRHandTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRHandTest_BoolMonitor {
    __cordl_parent: crate::System::Object,
    pub m_name: *mut crate::System::String,
    pub m_generator: *mut crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator,
    pub m_prevValue: bool,
    pub m_currentValue: bool,
    pub m_currentValueRecentlyChanged: bool,
    pub m_displayTimeout: f32,
    pub m_displayTimer: f32,
}
#[cfg(feature = "OVRHandTest+BoolMonitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRHandTest_BoolMonitor => ""
    ."OVRHandTest/BoolMonitor"
);
#[cfg(feature = "OVRHandTest+BoolMonitor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRHandTest_BoolMonitor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRHandTest_BoolMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor")]
impl crate::GlobalNamespace::OVRHandTest_BoolMonitor {
    #[cfg(feature = "OVRHandTest+BoolMonitor+BoolGenerator")]
    pub type BoolGenerator = crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator;
    pub fn AppendToStringBuilder(
        &mut self,
        sb: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendToStringBuilder", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        generator: *mut crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        generator: *mut crate::GlobalNamespace::BoolMonitor_OVRHandTest_BoolGenerator,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRHandTest+BoolMonitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRHandTest_BoolMonitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
