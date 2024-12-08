#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BoolMonitor_BoolGenerator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BoolMonitor_BoolGenerator => ""
    ."OVRControllerTest/BoolMonitor/BoolGenerator"
);
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::BoolMonitor_BoolGenerator {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BoolMonitor_BoolGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl crate::GlobalNamespace::BoolMonitor_BoolGenerator {
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
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
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
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BoolMonitor_BoolGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerTest_BoolMonitor {
    __cordl_parent: crate::System::Object,
    pub m_name: *mut crate::System::String,
    pub m_generator: *mut crate::GlobalNamespace::BoolMonitor_BoolGenerator,
    pub m_prevValue: bool,
    pub m_currentValue: bool,
    pub m_currentValueRecentlyChanged: bool,
    pub m_displayTimeout: f32,
    pub m_displayTimer: f32,
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRControllerTest_BoolMonitor
    => ""."OVRControllerTest/BoolMonitor"
);
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    #[cfg(feature = "OVRControllerTest+BoolMonitor+BoolGenerator")]
    pub type BoolGenerator = crate::GlobalNamespace::BoolMonitor_BoolGenerator;
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        generator: *mut crate::GlobalNamespace::BoolMonitor_BoolGenerator,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        name: *mut crate::System::String,
        generator: *mut crate::GlobalNamespace::BoolMonitor_BoolGenerator,
        displayTimeout: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, generator, displayTimeout))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRControllerTest+BoolMonitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerTest_BoolMonitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRControllerTest")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerTest {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub uiText: *mut crate::UnityEngine::UI::Text,
    pub monitors: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRControllerTest_BoolMonitor,
    >,
    pub data: *mut crate::System::Text::StringBuilder,
}
#[cfg(feature = "OVRControllerTest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRControllerTest => ""."OVRControllerTest"
);
#[cfg(feature = "OVRControllerTest")]
impl std::ops::Deref for OVRControllerTest {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest")]
impl std::ops::DerefMut for OVRControllerTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerTest")]
impl OVRControllerTest {
    #[cfg(feature = "OVRControllerTest+BoolMonitor")]
    pub type BoolMonitor = crate::GlobalNamespace::OVRControllerTest_BoolMonitor;
    #[cfg(feature = "OVRControllerTest+__c")]
    pub type __c = crate::GlobalNamespace::OVRControllerTest___c;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRControllerTest")]
impl quest_hook::libil2cpp::ObjectType for OVRControllerTest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
