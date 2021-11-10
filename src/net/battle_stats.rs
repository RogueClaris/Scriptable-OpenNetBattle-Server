#[derive(Debug)]
pub struct NPCBattleStats {
  pub id: String,
  pub health: u32,
}

#[derive(Debug)]
pub struct BattleStats {
  pub health: u32,
  pub score: u32,
  pub time: f32,
  pub ran: bool,
  pub emotion: u8,
  pub turns: u32,
  pub npcs: Vec<NPCBattleStats>,
}
