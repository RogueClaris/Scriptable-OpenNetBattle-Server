use super::Navi;
use crate::packets::PacketShipper;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::net::SocketAddr;

pub(super) struct Client {
  pub socket_address: SocketAddr,
  pub packet_shipper: PacketShipper,
  pub navi: Navi,
  pub warp_in: bool,
  pub warp_x: f32,
  pub warp_y: f32,
  pub warp_z: f32,
  pub ready: bool,
  pub cached_assets: HashSet<String>,
  pub texture_buffer: Vec<u8>,
  pub animation_buffer: Vec<u8>,
  message_queue: VecDeque<usize>, // for tracking what plugin sent the message this response is for
}

impl Client {
  pub(super) fn new(
    socket_address: SocketAddr,
    name: String,
    area_id: String,
    spawn_x: f32,
    spawn_y: f32,
    spawn_z: f32,
    resend_budget: usize,
  ) -> Client {
    use super::asset::{get_player_animation_path, get_player_texture_path};
    use uuid::Uuid;

    let id = Uuid::new_v4().to_string();

    Client {
      socket_address,
      packet_shipper: PacketShipper::new(socket_address, resend_budget),
      navi: Navi {
        id: id.clone(),
        name,
        area_id,
        texture_path: get_player_texture_path(&id),
        animation_path: get_player_animation_path(&id),
        x: spawn_x,
        y: spawn_y,
        z: spawn_z,
        solid: false,
      },
      warp_in: true,
      warp_x: spawn_x,
      warp_y: spawn_y,
      warp_z: spawn_z,
      ready: false,
      cached_assets: HashSet::new(),
      texture_buffer: Vec::new(),
      animation_buffer: Vec::new(),
      message_queue: VecDeque::new(),
    }
  }

  pub(super) fn track_message(&mut self, owner: usize) {
    self.message_queue.push_back(owner);
  }

  pub(super) fn pop_message(&mut self) -> Option<usize> {
    self.message_queue.pop_back()
  }
}
