use cxx::{UniquePtr, let_cxx_string};

use booster_robot_sys::robot::{
    b1::{LocoApiId, api_const::HandAction, b1_loco_client_new},
    common::RobotMode,
};

use crate::error::{Result, ToResult};

pub struct B1LocoClient {
    inner: UniquePtr<booster_robot_sys::robot::b1::B1LocoClient>,
}

impl B1LocoClient {
    pub fn new() -> Self {
        Self {
            inner: b1_loco_client_new(),
        }
    }

    pub fn send_api_request(&mut self, api_id: LocoApiId, param: &str) -> Result {
        let_cxx_string!(param_cxx = param);

        self.inner
            .pin_mut()
            .SendApiRequest(api_id, &param_cxx)
            .to_result()
    }

    pub fn change_mode(&mut self, mode: RobotMode) -> Result {
        self.inner.pin_mut().ChangeMode(mode).to_result()
    }

    #[doc(alias = "move")]
    pub fn move_with(&mut self, x_velocity: f32, y_velocity: f32, yaw_velocity: f32) -> Result {
        self.inner
            .pin_mut()
            .Move(x_velocity, y_velocity, yaw_velocity)
            .to_result()
    }

    pub fn rotate_head(&mut self, pitch: f32, yaw: f32) -> Result {
        self.inner.pin_mut().RotateHead(pitch, yaw).to_result()
    }

    pub fn wave_hand(&mut self, action: HandAction) -> Result {
        self.inner.pin_mut().WaveHand(action).to_result()
    }

    pub fn lie_down(&mut self) -> Result {
        self.inner.pin_mut().LieDown().to_result()
    }

    pub fn get_up(&mut self) -> Result {
        self.inner.pin_mut().GetUp().to_result()
    }
}

impl Default for B1LocoClient {
    fn default() -> Self {
        Self::new()
    }
}
