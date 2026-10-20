use std::sync::Arc;

use pumpkin_data::entity::EntityType;
use pumpkin_data::tag::Enchantment as EnchantmentTag;
use pumpkin_macros::pumpkin_block_from_tag;
use pumpkin_util::GameMode;

use crate::block::BlockBehaviour;
use crate::block::BrokenArgs;
use crate::enchantment::EnchantmentHelper;
use crate::entity::Entity;

#[pumpkin_block_from_tag("c:cobblestones/infested")]
pub struct InfestedBlock;

impl BlockBehaviour for InfestedBlock {
    fn broken(&self, args: BrokenArgs<'_>) {
        {
            // TODO: ugly fix, use onStacksDropped
            if args.player.gamemode.load() == GameMode::Creative {
                return;
            }
            if EnchantmentHelper::has_tag(
                &args.player.inventory().held_item(),
                &EnchantmentTag::MINECRAFT_PREVENTS_INFESTED_SPAWNS,
            ) {
                return;
            }
            let entity = Entity::new(
                args.world.clone(),
                args.position.0.to_f64(),
                &EntityType::SILVERFISH,
            );

            args.world.spawn_entity(Arc::new(entity));
        }
    }
}
