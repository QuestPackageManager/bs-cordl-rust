#[cfg(feature = "Zenject+SignalSubscriptionId")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SignalSubscriptionId {
    pub _signalId: crate::Zenject::BindingId,
    pub _callback: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+SignalSubscriptionId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalSubscriptionId => "Zenject"
    ."SignalSubscriptionId"
);
#[cfg(feature = "Zenject+SignalSubscriptionId")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Zenject::SignalSubscriptionId {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Zenject+SignalSubscriptionId")]
impl crate::Zenject::SignalSubscriptionId {
    pub fn Equals_Il2CppObject0(
        &mut self,
        that: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (that),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_SignalSubscriptionId1(
        &mut self,
        that: crate::Zenject::SignalSubscriptionId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (that),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        signalId: crate::Zenject::BindingId,
        callback: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (signalId, callback),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Callback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Callback",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_SignalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingId> {
        let __cordl_ret: crate::Zenject::BindingId = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SignalId",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
