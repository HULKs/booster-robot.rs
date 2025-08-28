pub mod api_const;

pub use ffi::*;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum LocoApiId {
        kChangeMode = 2000,
        kMove = 2001,
        kRotateHead = 2004,
        kWaveHand = 2005,
        kRotateHeadWithDirection = 2006,
        kLieDown = 2007,
        kGetUp = 2008,
        kMoveHandEndEffector = 2009,
        kControlGripper = 2010,
        kGetFrameTransform = 2011,
        kSwitchHandEndEffectorControlMode = 2012,
        kControlDexterousHand = 2013,
        kHandshake = 2015,
        kDance = 2016,
        kGetMode = 2017,
    }

    unsafe extern "C++" {
        include!("wrapper.hpp");

        type RobotMode = crate::robot::common::RobotMode;
        type HandAction = crate::robot::b1::api_const::HandAction;

        type B1LocoClient;
        type LocoApiId;

        #[cxx_name = "construct_unique"]
        fn b1_loco_client_new() -> UniquePtr<B1LocoClient>;

        fn Init(self: Pin<&mut B1LocoClient>);

        fn SendApiRequest(
            self: Pin<&mut B1LocoClient>,
            api_id: LocoApiId,
            param: &CxxString,
        ) -> i32;

        // fn SendApiRequestWithResponse(
        //     self: Pin<&mut B1LocoClient>,
        //     api_id: LocoApiId,
        //     param: &CxxString,
        //     response: &mut Response,
        // ) -> i32;

        fn ChangeMode(self: Pin<&mut B1LocoClient>, mode: RobotMode) -> i32;

        fn Move(self: Pin<&mut B1LocoClient>, vx: f32, vy: f32, vyaw: f32) -> i32;

        fn RotateHead(self: Pin<&mut B1LocoClient>, pitch: f32, yaw: f32) -> i32;

        fn WaveHand(self: Pin<&mut B1LocoClient>, action: HandAction) -> i32;

        fn LieDown(self: Pin<&mut B1LocoClient>) -> i32;

        fn GetUp(self: Pin<&mut B1LocoClient>) -> i32;
    }
}
