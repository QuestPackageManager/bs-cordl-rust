#[cfg(feature = "System+DateTimeParse+DS")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_DS {
    BEGIN = 0i32,
    DX_DS = 26i32,
    DX_DSN = 27i32,
    DX_MN = 23i32,
    DX_MNN = 25i32,
    DX_NDS = 28i32,
    DX_NM = 24i32,
    DX_NN = 21i32,
    DX_NNDS = 29i32,
    DX_NNN = 22i32,
    DX_NNY = 38i32,
    DX_YM = 33i32,
    DX_YMN = 31i32,
    DX_YN = 32i32,
    DX_YNN = 30i32,
    D_M = 6i32,
    D_MN = 7i32,
    D_MNd = 9i32,
    D_NDS = 10i32,
    D_NM = 8i32,
    D_NN = 4i32,
    D_NNd = 5i32,
    D_Nd = 3i32,
    D_S = 16i32,
    D_Y = 11i32,
    D_YM = 14i32,
    D_YMd = 15i32,
    D_YN = 12i32,
    D_YNd = 13i32,
    ERROR = 20i32,
    N = 1i32,
    NN = 2i32,
    TX_N = 34i32,
    TX_NN = 35i32,
    TX_NNN = 36i32,
    TX_TS = 37i32,
    T_NNt = 19i32,
    T_Nt = 18i32,
    T_S = 17i32,
}
#[cfg(feature = "System+DateTimeParse+DS")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_DS => "System"
    ."DateTimeParse/DS"
);
#[cfg(feature = "System+DateTimeParse+DTT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_DTT {
    DayOfWeek = 11i32,
    End = 0i32,
    Era = 16i32,
    Max = 20i32,
    MonthDatesep = 8i32,
    MonthEnd = 6i32,
    MonthSpace = 7i32,
    NumAmpm = 2i32,
    NumDatesep = 4i32,
    NumDatesuff = 9i32,
    NumEnd = 1i32,
    NumLocalTimeMark = 19i32,
    NumSpace = 3i32,
    NumTimesep = 5i32,
    NumTimesuff = 10i32,
    NumUTCTimeMark = 17i32,
    TimeZone = 15i32,
    Unk = 18i32,
    YearDateSep = 13i32,
    YearEnd = 14i32,
    YearSpace = 12i32,
}
#[cfg(feature = "System+DateTimeParse+DTT")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_DTT => "System"
    ."DateTimeParse/DTT"
);
#[cfg(feature = "System+DateTimeParse")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+DateTimeParse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse => "System"
    ."DateTimeParse"
);
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::Deref for crate::System::DateTimeParse {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::DerefMut for crate::System::DateTimeParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl crate::System::DateTimeParse {
    #[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
    pub type MatchNumberDelegate = crate::System::DateTimeParse_MatchNumberDelegate;
    #[cfg(feature = "System+DateTimeParse+DS")]
    pub type DS = crate::System::DateTimeParse_DS;
    #[cfg(feature = "System+DateTimeParse+DTT")]
    pub type DTT = crate::System::DateTimeParse_DTT;
    #[cfg(feature = "System+DateTimeParse+__c")]
    pub type __c = crate::System::DateTimeParse___c;
    #[cfg(feature = "System+DateTimeParse+TM")]
    pub type TM = crate::System::DateTimeParse_TM;
}
#[cfg(feature = "System+DateTimeParse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeParse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse_MatchNumberDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_MatchNumberDelegate =>
    "System"."DateTimeParse/MatchNumberDelegate"
);
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::Deref for crate::System::DateTimeParse_MatchNumberDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::DerefMut for crate::System::DateTimeParse_MatchNumberDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl crate::System::DateTimeParse_MatchNumberDelegate {
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
    pub fn Invoke(
        &mut self,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (str, digitLen, result))?;
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
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::DateTimeParse_MatchNumberDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DateTimeParse+TM")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_TM {
    AM = 0i32,
    NotSet = -1i32,
    PM = 1i32,
}
#[cfg(feature = "System+DateTimeParse+TM")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_TM => "System"
    ."DateTimeParse/TM"
);
